use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Ip {
    origin: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let res = client
        .get("https://httpbin.org/ip")
        .send()
        .await?;

    let ip = res
        .json::<Ip>()
        .await?;

    println!("Meu IP público: {}", ip.origin);
    Ok(())
}
