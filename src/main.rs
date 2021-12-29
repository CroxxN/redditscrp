mod mongo;
use crate::mongo::{DB,doc};
use serde::{Serialize, Deserialize};

// A reddit scapper for getting insults from r/insults using reqwest and tokio async runtime
#[derive(Serialize, Deserialize, Debug)]
struct DocFormat{
    title: String,
    content: String,
}
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
    let mut resp_insults: Vec<String> = Vec::new();
    for i in 0..200 {
        
        let resp = response.as_object().map(|map_res|{map_res["data"]["children"][i]["data"]["title"].to_string()});
        match resp {
            Some(resp)=> {resp_insults.push(resp);}
            None => {println!("Error");}
        };
        
        
    };
    let mut all_insult:Vec<mongodb::bson::Document> = Vec::new();
    for items in resp_insults {
        let doc_resp = doc!{
            "title": "insult",
            "content": items,
        };
        all_insult.push(doc_resp);
    };
    match mongo(all_insult).await {
        Ok(())=> println!("Susscess"),
        Err(_e) => println!("Error"),
    };
    Ok(())
}
async fn mongo(insult: Vec<mongodb::bson::Document>)->mongodb::error::Result<()>{
    let client= DB::init().await?;
    client.insert(insult).await?;
    Ok(())
}