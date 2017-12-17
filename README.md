# Docbase-cli
This package provides a unified command line [DocBase](https://docbase.io/).

Attention!

Now the version is 0.1. If you have an idea to improve it, Please share ideas in Issues.

## How to use

```
$ docbase-cli -h

DocBase API Command Line Interface Application

USAGE:
    docbase-cli
    docbase-cli post <post-file-path>... <post-title>...
    docbase-cli (-h | --help)
    docbase-cli --version

Options:
    -h, --help      Show this screen.
    --version       Show version.
```

## Installation
To install, you need to setup rust environment.

```
$ git clone git@github.com:Khigashiguchi/docbase-cli.git
$ cd docbase-cli
$ cargo build
```

## Getting Started
Before using docbase-cli, you need to tell it about your DOCBASE credentials.  
You can do this in several ways

- Environment variables
- Config file(developing)

To use environment variables do the follwing:

```
$ echo DOCBASE_TOKEN=<api-token>
```

