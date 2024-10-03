use mongodb::{bson, error::Error};

use crate::connection::database::Database;

use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub password: String,   
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
        }
    }
    pub fn from_credentials(credentials: &str) -> Self {
        let credentials: Vec<&str> = credentials.split(":").collect();
        Self {
            username: credentials[1].to_string(),
            password: credentials[2].to_string(),
        }
    }

    pub async fn insert_user(database: &Database, user: &User) -> Result<(), mongodb::error::Error> {
        let collection = database.get_collection("chat", "users").unwrap();
        let user_doc = bson::to_document(user).unwrap();
        
        // Check if username already exists
        let filter = bson::doc! { "username": &user.username };
        if collection.find_one(filter).await?.is_some() {
            return Err(Error::from(Error::custom("Username already exists")));
        }

        collection.insert_one(user_doc).await?;
        Ok(())
    }
}