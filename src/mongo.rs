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
        let db_user = env::var("MONGO_DB_USER");
        let db_pass = env::var("MONG0_DB_PASSWORD");
        let mut client_options = ClientOptions::parse(format!("mongodb+srv://{:?}:{:?}@insults.l3jlv.mongodb.net/myFirstDatabase?retryWrites=true&w=majority",db_user,db_pass))
        .await?;
        client_options.app_name = Some("insults".to_string());
        let client = Client::with_options(client_options)?;
        Ok(
            Self {
                client: client,
            }
        )
            
    }
}
