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

|               | Github repos | Gitlab repos |
| ------------- | ------------ | ------------ |
| Github issues | ✅           | ✅           |
| Asana         | ✅           | ✅           |
| Jira          | ✅           | ✅           |
| ClickUp       | ✅           | ✅           |

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

export CLICKUP_TOKEN="PUT_YOUR_TOKEN_HERE" # read-only token used to access your click up tasks

export GITHUB_TOKEN="PUT_YOUR_TOKEN_HERE" # read-only token used to access your github issues
```

After that, do not forget to run `source ~/.zshrc` (or `source ~/.bashrc` if you use Bash). 🤓

Here are some hints on how to generate the tokens.

- **JIRA_USERNAME**: should be the email associated with your jira account
- [JIRA_TOKEN](https://support.atlassian.com/atlassian-account/docs/manage-api-tokens-for-your-atlassian-account/)
- [GITHUB_TOKEN](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-personal-access-token-classic)
- [CLICKUP_TOKEN](https://developer.clickup.com/docs/authentication#personal-token)

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

This command helps you to create a `mrburns.config.json` file

```bash
mrburns config
```

Pass `--global` to write to your user-level config instead of the project-local one:

```bash
mrburns config --global
```

mrburns reads configuration from two locations, layered in this order (later overrides earlier):

1. Built-in defaults
2. User-level config: `$XDG_CONFIG_HOME/mrburns/mrburns.config.json`, or `~/.config/mrburns/mrburns.config.json` if `XDG_CONFIG_HOME` is unset
3. Project-local config: `./mrburns.config.json` in the current working directory

The merge is deep per key, so a project file only needs to override the fields it cares about; everything else falls through to the user-level file and then the defaults. Use `mrburns config --get` to inspect the effective merged configuration.

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
