# Serverless Rust Example

This is an example of using [serverless-rust](https://www.serverless.com/plugins/serverless-rust). It contains a simple `hello` lambda function 

## Installation

1. Clone the repo:
    ```bash
    git clone git@github.com:SerheyDolgushev/serverless-rust-example.git
    cd serverless-rust-example
    ```
2. Install Node.js dependencies:
    ```bash
    npm install --only=dev
    ```

## Usage

[serverless-rust](https://www.serverless.com/plugins/serverless-rust)  allows invoking rust lambda functions locally. Please run following command to run `hello` function with `{"firstName": "Sherlock", "lastName": "Holmes"}` payload:

```bash
npx serverless invoke local -f hello \
    -d '{"action": "greet", "firstName": "Sherlock"}' \
    --env RUST_LOG=warn,lambda=debug
```

The output will contain function logs and response:
```bash
...
[2022-04-12T14:38:59Z DEBUG lambda] Event data: {"action":"greet","firstName":"Sherlock"}
...

{"greeting":"Hello, Sherlock!"}

```

## Known issues

Please be aware that local invoke will be stuck if the rust function is panicking ([AWS Local Invocation is stuck when rust is panicking](https://github.com/serverless/serverless/issues/10911)). The simplest way to reproduce it is to call:

```bash
npx serverless invoke local -f hello -d '{action": "panic"}'
```

It can be fixed by manually killing the docker container by running the following command in a new terminal:

```bash
docker stop $(docker ps --filter "ancestor=sls-docker-provided" -q)
```

Once the docker container is killed the previously stuck function will be done, and its output will be printed:

```bash
...
thread 'main' panicked at 'Panic: message!', src/main.rs:19:20
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
...
```