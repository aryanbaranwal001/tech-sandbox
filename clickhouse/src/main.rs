use clickhouse::{Client, Row};
use fake::Fake;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Row, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    email: String,
    age: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default()
        .with_url("http://localhost:8123")
        .with_database("users_db")
        .with_user("user")
        .with_password("admin");

    // await here because this sends query across network and executes the query
    // db takes time to respond
    client
        .query(
            "CREATE TABLE IF NOT EXISTS users (
                id String,
                name String,
                email String,
                age UInt8
            ) ENGINE = MergeTree() 
            ORDER BY id",
        )
        .execute()
        .await?;

    println!("Table 'users' is ready.");

    // establishes connection stream for insertion
    // clickhouse prepares to get data
    let mut insert = client.insert::<User>("users").await?;

    for _ in 0..3 {
        let user = User {
            id: Uuid::new_v4().to_string(),
            name: Name().fake(),
            email: SafeEmail().fake(),
            age: (18..65).fake(),
        };

        // adds rows to buffer and may send chunks over network
        insert.write(&user).await?;
    }

    // flushes remainig buffer, db cnofirms insert completel
    insert.end().await?;

    println!("Successfully inserted 10 fake users.");

    Ok(())
}
