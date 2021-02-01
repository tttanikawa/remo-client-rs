use dotenv::dotenv;
use remo_client_rs::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let token =
        env::var("NATURE_REMO_ACCESS_TOKEN").expect("NATURE_REMO_ACCESS_TOKEN is not defined");
    let client = Client::new(token);

    let user = client.get_user().await?;
    println!("{:?}", user);

    let devices = client.get_devices().await?;
    println!("{:?}", devices);

    Ok(())
}
