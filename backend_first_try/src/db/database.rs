use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::models::sushi::Sushi;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;

        client.use_ns("surreal").use_db("sushi").await.unwrap();

        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("sushi"),
        })
    }

    pub async fn get_all_sushi(&self) -> Option<Vec<Sushi>> {
        // Database struct の clientを見に行って、
        let result = self.client.select("sushi").await;
        match result {
            Ok(all_sushi) => Some(all_sushi),
            Err(_) => None,
        }
    }

    pub async fn add_new_sushi(&self, new_sushi: Sushi) -> Option<Sushi> {
        let added_sushi = self
            .client
            .create(("sushi", new_sushi.uuid.clone()))
            .content(new_sushi)
            .await;
        match added_sushi {
            Ok(s) => s,
            Err(e) => {
                println!("error occurred: {:?}", e);
                None
            }
        }
    }
}
