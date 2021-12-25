use miette::{Diagnostic, SourceOffset, SourceSpan};
use relative_path::RelativePath;
use serde::Serialize;
use std::{path::Path, process::Command};
use thiserror::Error;
use tinytemplate::error::Error as TemplateError;
use tinytemplate::TinyTemplate;

use super::DerivativeConfig;

#[derive(Serialize)]
struct Context<'a> {
    path: &'a RelativePath,
}

#[derive(Error, Diagnostic, Debug)]
pub enum UpdateError {
    #[error(transparent)]
    #[diagnostic(code(tinytemplate::error::Error))]
    TemplateError(#[from] tinytemplate::error::Error),

    #[error("Failed to parse the template")]
    #[diagnostic(code(tinytemplate::error::Error::ParseError))]
    ParseError {
        source: tinytemplate::error::Error,
        #[source_code]
        code: String,

        #[label = "in this code..."]
        defined_span: SourceSpan,
        #[label = "around here"]
        span: SourceSpan,
    },

    #[error(transparent)]
    #[diagnostic(code(std::io::Error))]
    IoError(#[from] std::io::Error),

    #[error("A generator command failed to run.")]
    #[diagnostic(
        code(git_derivative::derivatives::updater::UpdateError::CommandError),
        help("you may want to check the command")
    )]
    CommandError {
        #[source_code]
        code: String,
        #[label("command defind here")]
        command_span: SourceSpan,
    },
}

fn spanned_to_source_span(span: &toml::Spanned<String>) -> SourceSpan {
    (span.start(), span.end() - span.start()).into()
}

fn calculate_offset(
    generator: &str,
    span: &toml::Spanned<String>,
    line: usize,
    col: usize,
) -> SourceOffset {
    let base = spanned_to_source_span(span).offset();
    let offset = SourceOffset::from_location(generator, line, col + 3).offset();
    (base + offset).into()
}

pub fn run_config(
    config: &DerivativeConfig,
    root: &Path,
    requested_paths: &[&RelativePath],
    source: String,
) -> Result<(), UpdateError> {
    for &path in requested_paths {
        let mut template = TinyTemplate::new();

        let generator = if let Some(v) = config.generators.get(path) {
            v.get_ref()
        } else {
            continue;
        };

        template
            .add_template(path.as_str(), generator)
            .or_else(|err| match err {
                TemplateError::ParseError { column, line, .. } => {
                    let spanned = config.generators.get(path).cloned().unwrap();
                    let start = calculate_offset(generator, &spanned, line, column);
                    Err(UpdateError::ParseError {
                        source: err,
                        code: source.clone(),
                        defined_span: spanned_to_source_span(&spanned),
                        span: (start, 0.into()).into(),
                    })
                }
                others => Err(others.into()),
            })?;

        let command = template.render(path.as_str(), &Context { path })?;

        #[cfg(not(target_os = "windows"))]
        let result = Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(root)
            .status()?;
        #[cfg(target_os = "windows")]
        let result = Command::new("cmd")
            .arg("/C")
            .arg(command)
            .current_dir(root)
            .status()?;

        if !result.success() {
            return Err(UpdateError::CommandError {
                code: source,
                command_span: spanned_to_source_span(&config.generators.get(path).unwrap()),
            });
        }
    }
    Ok(())
}

pub fn run_all_config(
    config: &DerivativeConfig,
    root: &Path,
    source: String,
) -> Result<(), UpdateError> {
    for (path, generator) in config.generators.iter() {
        let mut template = TinyTemplate::new();
        template
            .add_template(path.as_str(), generator.get_ref())
            .or_else(|err| match err {
                TemplateError::ParseError { column, line, .. } => {
                    let spanned = config.generators.get(path).cloned().unwrap();
                    let start =
                        calculate_offset(generator.get_ref(), &spanned, line, column);
                    Err(UpdateError::ParseError {
                        source: err,
                        code: source.clone(),
                        defined_span: spanned_to_source_span(&spanned),
                        span: (start, 0.into()).into(),
                    })
                }
                others => Err(others.into()),
            })?;

        let command = template.render(path.as_str(), &Context { path })?;

        #[cfg(not(target_os = "windows"))]
        let result = Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(root)
            .status()?;
        #[cfg(target_os = "windows")]
        let result = Command::new("cmd")
            .arg("/C")
            .arg(command)
            .current_dir(root)
            .status()?;

        if !result.success() {
            return Err(UpdateError::CommandError {
                code: source,
                command_span: spanned_to_source_span(&config.generators.get(path).unwrap()),
            });
        }
    }
    Ok(())
}
