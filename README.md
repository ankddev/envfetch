<h1 align="center">envfetch</h1>
<h5 align="center">Lightweight cross-platform CLI tool for working with environment variables</h5>
<div align="center">
    <a href="https://github.com/ankddev/envfetch/actions/workflows/build.yml"><img src="https://github.com/ankddev/envfetch/actions/workflows/build.yml/badge.svg" alt="Build status"/></a>
    <a href="https://github.com/ankddev/envfetch/actions/workflows/test.yml"><img src="https://github.com/ankddev/envfetch/actions/workflows/test.yml/badge.svg" alt="Test status"/></a>
    <img alt="GitHub commit activity" src="https://img.shields.io/github/commit-activity/w/ankddev/envfetch">
    <a href="https://app.codecov.io/github/ankddev/envfetch"><img src="https://camo.githubusercontent.com/24e6fbb5fab320f1c87a360fcdf93b0901a6fc04fe0cb070a4083346c7946680/68747470733a2f2f636f6465636f762e696f2f67682f616e6b646465762f656e7666657463682f67726170682f62616467652e7376673f746f6b656e3d37325138463858574b51" /></a>
    <a href="https://crates.io/crates/envfetch"><img src="https://img.shields.io/crates/d/envfetch" alt="crates.io downloads"/></a>
    <a href="https://crates.io/crates/envfetch"><img src="https://img.shields.io/crates/v/envfetch" alt="crates.io version"/></a>
    <a href="https://aur.archlinux.org/packages/envfetch"><img src="https://img.shields.io/aur/version/envfetch" alt="AUR version"/></a>
</div>
<div align="center">
    <img src="https://github.com/user-attachments/assets/261ea1fd-438a-40b0-847d-6a460b7a30a9" />
</div>

# Features
- [x] Print list of all environment variables
- [x] Get value of variable by name
    - [x] Show similar variables if given variable not found
- [x] Set variable (temporary and permanent)
- [x] Delete variable (temporary and permanent)
- [x] Load variables from dotenv-style file (temporary and permanent)
- [x] Add string to the end of variable (temporary and permanent)
- [ ] Set and delete multiple variables at once
- [ ] Interactive mode
  - [x] Basic support
- [ ] Export variables
- [ ] Configuration support
# Get started
## Installing

<a href="https://repology.org/project/envfetch/versions">
    <img src="https://repology.org/badge/vertical-allrepos/envfetch.svg" alt="Packaging status">
</a>

Read about installing `envfetch` in the [Wiki](https://github.com/ankddev/envfetch/wiki/2.-Installation).
## Using
Read in [Wiki](https://github.com/ankddev/envfetch/wiki/3.-Basic-Usage).
## Configuration
`envfetch` support some configuration. Fitsly, you need to run `envfetch init-config` to create config file, it will return you path of config.
| Platform |                       Path                        |
| -------- | ------------------------------------------------- |
| Windows  |  `C:\Users\<USER>\AppData\Roaming\envfetch.toml`  |
|  Linux   |           `$HOME/.config/envfetch.toml`           |
|  macOS   | `$HOME/Library/Application Support/envfetch.toml` |
### Keys
- `print_format` - Format string for print command
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
# See Also
- [codewars-api-rs](https://github.com/ankddev/codewars-api-rs) - Rust library for Codewars API
- [conemu-progressbar-go](https://github.com/ankddev/conemu-progressbar-go) - Progress bar for ConEmu for Go
- [terminal-go](https://github.com/ankddev/terminal-go) - Go library for working with ANSI/VT terminal sequences
- [zapret-discord-youtube](https://github.com/ankddev/zapret-discord-youtube) - Zapret build for Windows for fixing Discord and YouTube in Russia or other services
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
