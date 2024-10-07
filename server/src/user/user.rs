use mongodb::{bson, error::Error};

use crate::connection::database::Database;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

    pub async fn select_user(database: &Database, user: &User) -> Result<Option<User>, mongodb::error::Error> {
        let collection = database.get_collection("chat", "users").unwrap();
        let filter = bson::doc! { "username": &user.username, "password": &user.password };
        let user = collection.find_one(filter).await?;
        match user {
            Some(user) => {
                let user = bson::from_document(user).unwrap();
                Ok(Some(user))
            },
            None => Ok(None),
        }
    }
}