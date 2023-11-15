use dotenv::dotenv;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URLを.envで設定してないで！");
    let db: DatabaseConnection = Database::connect(&database_url).await?;
    println!("データベースに接続できたで！");
    Ok(())
}