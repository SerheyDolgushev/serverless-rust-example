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
    -d '{"firstName": "Sherlock", "lastName": "Holmes"}' \
    --env RUST_LOG=warn,lambda=debug
```

The output will contain function logs and response:
```bash
...
[2022-03-25T08:52:05Z DEBUG lambda] Event data: {"firstName":"Sherlock","lastName":"Holmes"}
...

{"message":"Hello, Sherlock!"}

```