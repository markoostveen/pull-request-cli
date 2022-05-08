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
    Use --help to view all supported commands for cli tool
    usage:
        git_pull-requests_cli --help
        git_pull-requests_cli init
        git_pull-requests_cli view <OWNER> <PROJECT_NAME>
        git_pull-requests_cli view <OWNER> <PROJECT_NAME> <PULL-REQUEST_ID>
        git_pull-requests_cli comment <OWNER> <PROJECT_NAME> <PULL-REQUEST_ID> <MESSAGE>
```
