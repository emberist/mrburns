<p align="center">
  <img
    alt="logo"
    src="./mrburns.png"
    width="10%"
  />
</p>

<h1 align="center">mrburns CLI</h1>

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
export GITLAB_TOKEN="PUT_YOUR_TOKEN_HERE"
```

After that, do not forget to run `source ~/.zshrc` (or `source ~/.bashrc` if you use Bash). 🤓

Here are some hints on how to generate the tokens.

- **JIRA_USERNAME**: should be the email associated with your jira account
- [JIRA_TOKEN](https://support.atlassian.com/atlassian-account/docs/manage-api-tokens-for-your-atlassian-account/)
- [GITLAB_TOKEN](https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html)

## 🚀 Run

### Help

This command prints mrburns's help screen, which lists all the available commands and options.

```bash
mrburns --help
```

### Start a task

This commands switches to a new Git branch to start working on it. mrburns takes care of creating a proper name for the branch starting from the task title.

```bash
mrburns start https://your-domain.atlassian.net/browse/foo-3293
```

NOTE: There is an options `--type` option (or `-t`) to choose the task type between bugfix, chore or feature

### Create a Merge Request

This commands packs a Gitlab Merge Request.

```bash
mrburns mr
```

### Set your config file

This command helps you creating a `mrburns.config.json` file

```bash
mrburns config
```

The config file is something like this

```json
{
  "create_draft_mr": true,
  ...
}
```

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
