#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use mongodb::{Client, Collection};
    use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};

    use serde::{Deserialize, Serialize};

    use crate::logger::{log, LogLevel};

    #[derive(Serialize, Deserialize, Debug)]
    struct TestType {
        name: String,
        age: i32,
    }

    #[tokio::test]
    async fn test_insert() -> mongodb::error::Result<()> {
        dotenv().ok();

        let connection_string = dotenv::var("MONGO_CONNECTION_STRING").expect("MONGODB_URI must be set");
        if connection_string.is_empty() {
            panic!("MONGO_CONNECTION_STRING must be set");
        }

        let mut client_options = ClientOptions::parse(connection_string).await?;
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options)?;

        let db = client.database("admin");

        db.create_collection("testtest", None).await.expect("Failed to create collection");
        log(LogLevel::Info, "Collection created.").expect("Failed to log.");

        let collection: Collection<TestType> = db.collection("testtest");

        let test = TestType {
            name: "Test".to_string(),
            age: 20,
        };

        collection.insert_one(test, None).await.expect("Failed to insert document");

        collection.drop(None).await.expect("Failed to drop collection");

        Ok(())
    }
}