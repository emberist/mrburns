<p align="center">
  <img
    alt="logo"
    src="./mrburns.png"
    width="10%"
  />
</p>

<h1 align="center">mrburns CLI</h1>

<div align="center">
mrburns allows you to effortlessly create task-specific branches and merge requests in just seconds!
</div>

## Compatibile tools

|               | Github repos | Gitlab repos | Bitbucket repos |
| ------------- | ------------ | ------------ | --------------- |
| Github issues | ✅           | ✅           | 🚧 WIP          |
| Asana         | ✅           | ✅           | 🚧 WIP          |
| Jira          | ✅           | ✅           | 🚧 WIP          |

## ⚙️ Setup

### Installation

First, install rustup and cargo, [here](https://www.rust-lang.org/tools/install) the docs

then run this command

```bash
cargo install mrburns --git https://github.com/emberist/mrburns
```

or, clone this repo and add the required permission to the `install.sh` script

```bash
chmod a+x ./install.sh
```

Then run the `install.sh` script:

```bash
./install.sh
```

### Supported tools

### Set the environment variables

Based on which tool you are using, you have to add the following environment variables in your `~/.zshrc` file (or `~/.bashrc` if you use Bash).

```bash
export JIRA_USERNAME="PUT_YOUR_USERNAME_HERE"
export JIRA_TOKEN="PUT_YOUR_TOKEN_HERE"

export GITHUB_TOKEN="PUT_YOUR_TOKEN_HERE" # read-only token used to access your github issues
```

After that, do not forget to run `source ~/.zshrc` (or `source ~/.bashrc` if you use Bash). 🤓

Here are some hints on how to generate the tokens.

- **JIRA_USERNAME**: should be the email associated with your jira account
- [JIRA_TOKEN](https://support.atlassian.com/atlassian-account/docs/manage-api-tokens-for-your-atlassian-account/)
- [GITHUB_TOKEN](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-personal-access-token-classic)

## 🚀 Run

### Help

This command prints mrburns's help screen, which lists all the available commands and options.

```bash
mrburns --help
```

### Start a task

This commands switches to a new Git branch to start working on it. **mrburns** takes care of creating a proper name for the branch starting from the task title.

```bash
mrburns start https://your-domain.atlassian.net/browse/foo-3293
```

NOTE: There is an options `--type` option (or `-t`) to choose the task type between bugfix, chore or feature

### Create a Merge Request

This commands open for you a prefilled marge request page

```bash
mrburns mr
```

### List

Helpful when you need to quickly switch between active mrburns's branches;

```bash
mrburns list
```

### Browse

Helpful when you need to quickly open you task from the codebase current branch!

```bash
mrburns browse
```

### Set your config file

This command helps you creating a `mrburns.config.json` file

```bash
mrburns config
```

The default config file is something like this

```json
{
  "mr": {
    "defaultDraft": false,
    "titleTemplate": "{task_id}/{task_type}/{task_title}",
    "descriptionTemplate": [],
    "descriptionTemplatePath": "./templates/default.md"
  },
  "branchPrefixes": {
    "feature": "feat",
    "release": "release",
    "bugfix": "bugfix",
    "chore": "chore"
  }
}
```

if the `descriptionTemplatePath` file exists, it will override the `descriptionTemplate` option

## 🧑‍💻 Develop

### Run

```bash
cargo run -- <...commands>
```

### Compile

```bash
cargo build
```

### Test

```bash
cargo test
```
