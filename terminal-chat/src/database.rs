use mongodb::{error::Result, options::ClientOptions, Client};

pub struct Database {
    client: Option<Client>,
    connection: String,
}

impl Database {
    
    pub fn new(&self, connection: String) -> Self {
        Self {
            client: None,
            connection,
        }
    }

    pub async fn connect(&self) -> Result<Self> {
        let client_options = ClientOptions::parse(&self.connection).await.expect("Failed to parse client options");
        let client = Client::with_options(client_options).expect("Failed to create client");
        Ok(Self {
            client: Some(client),
            connection: self.connection.clone(),
        })
    }

    pub fn get_database(&self, database_name: &str) -> Option<mongodb::Database> {
        match &self.client {
            Some(client) => {
                Some(client.database(database_name))
            },
            None => None,
        }
    }
}