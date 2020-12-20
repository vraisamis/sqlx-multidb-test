use std::env;

use anyhow::Result;
use serde::Deserialize;
use sqlx::{query_as, Connection, PgConnection};

#[derive(Debug, Deserialize)]
struct Pg1Users {
    id: Option<i32>,
    name: Option<String>,
    atk: Option<i32>,
    def: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = env::var("DATABASE_URL")?;
    let mut connection = PgConnection::connect(&database_url).await?;
    let result: Vec<_> = query_as!(Pg1Users, "select * from users")
        .fetch_all(&mut connection)
        .await?;
    print_data(&result);
    Ok(())
}

fn print_data<T: std::fmt::Debug>(datum: &Vec<T>) {
    for (i, data) in datum.iter().enumerate() {
        println!("{}: {:?}", i, data);
    }
}
