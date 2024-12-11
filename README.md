<h1 align="center">envfetch</h1>
<h5 align="center">Lightweight cross-platform CLI tool for working with environment variables</h5>
<div align="center">
    <a href="https://github.com/ankddev/envfetch/actions/workflows/build.yml"><img src="https://github.com/ankddev/envfetch/actions/workflows/build.yml/badge.svg" alt="Build status"/></a>
    <a href="https://github.com/ankddev/envfetch/actions/workflows/test.yml"><img src="https://github.com/ankddev/envfetch/actions/workflows/test.yml/badge.svg" alt="Test status"/></a>
    <img alt="GitHub commit activity" src="https://img.shields.io/github/commit-activity/w/ankddev/envfetch">
    <a href="https://crates.io/crates/envfetch"><img src="https://img.shields.io/crates/d/envfetch" alt="crates.io downloads"/></a>
    <a href="https://crates.io/crates/envfetch"><img src="https://img.shields.io/crates/v/envfetch" alt="crates.io version"/></a>
</div>
<div align="center">
    <img src="https://github.com/user-attachments/assets/261ea1fd-438a-40b0-847d-6a460b7a30a9" />
</div>

# Features
- [x] Print all environment variables
- [x] Get value of variable by name
    - [x] Show similar variables if variable not found
- [x] Set variable
- [x] Delete variable
- [x] Load variables from dotenv-style file
- [x] Globally set variables
- [x] Globally delete variables
- [x] Globally load variables from dotenv-style file
- [ ] Set and delete multiple variables at once
# Get started
## Installing
You can install envfetch from Cargo (needs Rust installed):
```shell
$ cargo install envfetch
```
Also, you can install it from source (needs Rust installed):
```shell
$ cargo install --git https://github.com/ankddev/envfetch envfetch
```
Or, get binary from [GitHub Actions (needs GutHub account)](https://github.com/ankddev/envfetch/actions/) or [releases](https://github.com/ankddev/envfetch/releases/)
## Using
To run envfetch, run `envfetch <COMMAND> <ARGS>` in your terminal.
You can run `envfetch help` to see help message or `envfetch --version` to see program's version.
### Global flags
- `--exit-on-error`/`-e` - exit on any error

### Command list
#### Set
Set environment variable and run process.
> [!NOTE]
> For temporary commands (set, delete, load), variables are only modified for the current process run

Usage:
`envfetch set <KEY> <VALUE> <PROCESS>`, where:
- `KEY` - name of environment variable
- `VALUE` - value of environment variable
- `PROCESS` - name of process which you want to run

Options:
- `--help`/`-h` - show help message

For example:
```shell
$ envfetch set MY_VAR "Hello" "npm run"
```
It will set environment variable with name `MY_VAR` value "Hello" and start `npm run`

#### Print
Print all environment variables

Usage:
`envfetch print`

Options:
- `--help`/`-h` - show help message

For example:
```shell
$ envfetch print
SHELL = "powershell"
windir = "C:\\Windows"
SystemDrive = "C:"
SystemRoot = "C:\\Windows"
...
```
It will print all environment variables in format `VAR = "VALUE"`.
#### Get
Get value of environment variable

Usage:
`envfetch get <KEY>`, where:
- `KEY` - name of environment variable

Options:
- `--help`/`-h` - show help message
- `--no-similar-names`/`-s` - disable showing similar variables if variable not

For example:
```shell
$ envfetch get MY_VAR
"Hello"
```
It will print value of specified variable.
#### Delete
Delete variable and start process.
> [!NOTE]
> For temporary commands (set, delete, load), variables are only modified for the current process run

Usage:
`envfetch delete <KEY> <PROCESS>`, where:
- `KEY` - name of environment variable
- `PROCESS` - name of command to run

Options:
- `--help`/`-h` - show help message

For example:
```shell
$ envfetch delete MY_VAR "npm run"
```
It will delete variable `MY_VAR` and run `npm run` command.
#### Load
Load environment variables from dotenv-style file and run process.
> [!NOTE]
> For temporary commands (set, delete, load), variables are only modified for the current process run

Usage:
`envfetch load <PROCESS>`, where:
- `PROCESS` - name of process which you want to run

Options:
- `--help`/`-h` - show help message
- `--file <FILE>`/`-f <FILE>` - relative or absolute path to file to read variables from. Note that it must in .env format.
By default, program loads variables from `.env` file in current directory.

For example:
```shell
$ envfetch load "npm run"
$ envfetch load "npm run" --file ".env.debug"
```
It will load variables from `.env` or `.env.debug` and start `npm run`

#### Gset
Set environment variable permanently.

Usage:
`envfetch gset <KEY> <VALUE>`, where:
- `KEY` - name of environment variable
- `VALUE` - value of environment variable

Options:
- `--help`/`-h` - show help message

For example:
```shell
$ envfetch gset MY_VAR "Hello"
```
It will permanently set environment variable with name `MY_VAR` to value "Hello"

#### Gdelete
Delete environment variable permanently.

Usage:
`envfetch gdelete <KEY>`, where:
- `KEY` - name of environment variable

Options:
- `--help`/`-h` - show help message

For example:
```shell
$ envfetch gdelete MY_VAR
```
It will permanently delete variable `MY_VAR`.

#### Gload
Load environment variables from dotenv-style file permanently.

Usage:
`envfetch gload`, where:

Options:
- `--help`/`-h` - show help message
- `--file <FILE>`/`-f <FILE>` - relative or absolute path to file to read variables from. Note that it must be in .env format.
By default, program loads variables from `.env` file in current directory.

For example:
```shell
$ envfetch gload
$ envfetch gload --file ".env.debug"
```
It will permanently load variables from `.env` or `.env.debug` file.

> [!NOTE]
> For global commands (gset, gdelete, gload):
> - On Windows, variables are stored in the registry under HKEY_CURRENT_USER\Environment
> - On Unix-like systems, variables are stored in shell configuration files (.bashrc, .zshrc, or config.fish)
# Building from source
- Install Rust. If it already installed, update with
```shell
$ rustup update
```
- Fork this project using button `Fork` on the top of this page
- Clone your fork (replace `<YOUR_USERNAME>` with your username on GitHub):
```shell
$ git clone https://github.com/<YOUR_USERNAME>/envfetch.git
```
- Go to directory, where you cloned envfetch:
```shell
$ cd envfetch
```
- Run program using Cargo (replace `<COMMAND>` and `<ARGS>` to your command and args):
```shell
$ cargo run -- <COMMAND> <ARGS>
```
# Contributing
- Read [section above to build envfetch from source](#building-from-source)
- Create new branch
- Made your changes
- Test that everything works correctly
- Format and lint code with
```shell
$ cargo fmt
$ cargo clippy --fix
```
- Run tests with
```shell
$ cargo test
```
- Push changes
- Open pull request
