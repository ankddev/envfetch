# What's this?

This directory contains some assets and service files.

## Structure

```sh
assets
├── .env.example                    # Example .env file, used for demonstartion
├── assets/README.md                # This file
├── assets/default_config.toml      # Default config file, packs during compilation
├── assets/envfetch.ascii           # Text representation of utility's work. Will be used for integration tests later
├── assets/envfetch.gif             # Demonstration of utility's work
├── assets/envfetch.tape            # VHS tape, used to create GIF and images
├── assets/interactive_add.png      # Picture of creating variable in interactive mode
├── assets/interactive_main.png     # Picture of interactive mode main screen
└── assets/main.png                 # Picture of main commands
```

## How to record GIF

Firstly, [install VHS](https://github.com/charmbracelet/vhs#installation) and thhen from ROOT DIRECTORY of repo run following:

```sh
vhs assets/envfetch.tape
```

This will create all screenshots and GIF image.
