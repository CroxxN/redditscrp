mod mongo;
use crate::mongo::{DB};

// A reddit scapper for getting insults from r/insults using reqwest and tokio async runtime
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new(); //Creates a reqwest client
    // Creating a variable response with type of serde_json::Value so that
    //it holds arbitrairy json data
    let response: serde_json::Value = client 
    .get("https://www.reddit.com/r/insults/best/.json?limit=200")
    .send()
    .await?
    .json()
    .await?;
    let result_doc;
    for i in 0..200 {
        let response_formatted = response["data"]["children"][i]["data"]["title"].to_string();
        let result_doc = DB::make_doc(response_formatted);
    };
    match mongo(result_doc).await {
        Ok(resp)=> println!("{:?}",resp),
        Err(error) => println!("{}",error)
    };
    Ok(())
}
async fn mongo(insult: mongodb::bson::Document)->mongodb::error::Result<Option<(String,mongodb::bson::Bson)>>{
    let client= DB::init().await?;
    let response = client.insert(insult).await?;
    Ok(response)
}