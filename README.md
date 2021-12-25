# git-derivative
A git hook to manage derivative files automatically.

For example if you checked out to a branch with different `yarn.lock`, git-derivative can run `yarn install` for you!

## Installation

	$ git derivative init
	$ git derivative install # You may want to run this automatically for your project

You can write what to run in `.gitderivative` file.

```toml
[generators]
"yarn.lock": "yarn install"
```
