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

    let response = match event["action"].as_str().unwrap_or("greet") {
        "panic" => panic!("Panic: {}!", event["message"].as_str().unwrap_or("message")),
        "get_variable" => {
            let name = event["name"].as_str().unwrap_or("domain");
            let value = std::env::var(name).unwrap_or(String::from("NOT_FOUND"));
            json!({"name": name, "value": value})
        }
        _ => json!({ "greeting": format!("Hello, {}!", event["firstName"].as_str().unwrap_or("world")) })
    };

    Ok(response)
}