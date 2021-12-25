use mongodb::{options::ClientOptions,Client};
use dotenv::dotenv;
use std::env;
#[derive(Debug)]
pub struct DB{
    pub client: Client,
}
impl DB {
    pub async fn init()-> mongodb::error::Result<Self>{
        dotenv().ok();
        let db_user = env::var("MONGO_DB_USER").unwrap();
        let db_pass = env::var("MONG0_DB_PASSWORD").unwrap();
        println!("{:?}",db_user);
        let mut client_options = ClientOptions::parse("mongodb+srv://{}:{}@insults.l3jlv.mongodb.net/myFirstDatabase?retryWrites=true&w=majority")
        .await?;
        client_options.app_name = Some("insults".to_string());
        let client = Client::with_options(client_options)?;
        Ok(
            Self {
                client: client,
            }
        )
            
    }
    pub async fn insert(&self)-> mongodb::error::Result<u64>{
        let insult_collection: mongodb::Collection<String> = self.client.database("insults").collection("insults");
        let collection_amount = insult_collection.count_documents(None, None)
        .await?;
        Ok(collection_amount)
    }
}
