use std::{f32::consts::E, result::Result};
use sqlx::{migrate::MigrateDatabase, pool, sqlite::SqliteQueryResult, Sqlite, SqlitePool};

async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error>{
    let pool = SqlitePool::connect(&db_url).await?;
    let qry = r#"
    CREATE TABLE IF NOT EXISTS settings
    (
        settings_id     INTEGER PRIMARY KEY     NOT NULL,
        description     TEXT                    NOT NULL,
        created_on      DATETIME DEFAULT        (datetime('now', 'localtime')),
        updated_on      DATETIME DEFAULT        (datetime('now', 'localtime')),
        done            BOOLEAN                 NOT NULL DEFAULT 0
    );

    CREATE TABLE IF NOT EXISTS project
    (
        project_id      INTEGER     
    );
    "#;

    let result = sqlx::query(&qry).execute(&pool).await;

    pool.close().await;
    result     
}


#[async_std::main]
async fn main() {
    let db_url = String::from("sqlite://sqlite.db");
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();
        match create_schema(&db_url).await {
            Ok(_) => println!("Databaza yaradildi"),
            Err(e) => panic!("{}", e)
        }
    }
    let instance = SqlitePool::connect(&db_url).await.unwrap();
    let qry = "INSERT INTO settings (description) VALUES($1)";
    let result = sqlx::query(&qry).bind("testing").execute(&instance).await.unwrap();
    instance.close().await;
    println!("{:#?}", result)
}
