use std::env::var;
use std::error::Error;

use sqlx::MySqlPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database_url = var("DATABASE_URL")?;
    let _pool = MySqlPool::connect(&database_url).await?;
    println!("connected");
    Ok(())
}

#[cfg(test)]
#[sqlx::test]
async fn should_pass(_pool: MySqlPool) {
    tokio::task::yield_now().await;
}
