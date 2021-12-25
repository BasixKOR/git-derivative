use git2::Repository;

pub fn open_repo() -> Result<Repository, git2::Error> {
	let repo = Repository::open(".")?;
	Ok(repo)
}