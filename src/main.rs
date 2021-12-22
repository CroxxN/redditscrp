use serde;
#[Serialize, Deserialize]
struct 

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let resp = reqwest::get("https://www.reddit.com/r/insults/new/.json?limit=2")
    .await?;
    let body = resp.json().await?;
    println!("{}",body[0]);
    Ok(())
}