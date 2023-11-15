use dotenv::dotenv;
use log;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration; // Durationのインポート

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    dotenv().ok();
    let db = db_connect().await?;
    println!("データベースに接続できたで！");
    db.close().await?;
    Ok(())
}

async fn db_connect() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URLを.envで設定してないで！");
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(opt).await?;

    Ok(db)
}
