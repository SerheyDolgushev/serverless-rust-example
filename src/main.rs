use lambda_runtime::{Error, LambdaEvent, service_fn};
use serde_json::{json, Value};
use log::{debug};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();

    debug!("Event data: {}", event);

    let first_name = event["firstName"].as_str().unwrap_or("world");

    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}