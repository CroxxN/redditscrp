
// A reddit scapper for getting insults from r/insults using reqwest and tokio async runtime
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new(); //Creates a reqwest client
    // Creating a variable response with type of serde_json::Value so that
    //it holds arbitrairy json data
    let response: serde_json::Value = client 
    .get("https://www.reddit.com/r/insults/new/.json?limit=2")
    .send()
    .await?
    .json()
    .await?;
    println!("{:#?}", response["data"]["children"][1]["data"]["title"]);
    Ok(())
}