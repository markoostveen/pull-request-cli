# Cli-tool to list & update pull-requests for github projects

## How to build
in order to build, make sure rust is installed.
Then run in the root project directory.
```shell
cargo update
```

## How to use
run with --help flags to view usage at any time.

```
pull-request-cli 
tool to help view & update pull-requests on github projects

USAGE:
    git_pull-requests_cli.exe [SUBCOMMAND]

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    comment     Post a comment on an existing pull-request
    help        Print this message or the help of the given subcommand(s)
    identity    Print identity of logged in user
    init        Initialize cli-tool and connect to github using personal access token
    view        View pull-requests of a github repo
```
