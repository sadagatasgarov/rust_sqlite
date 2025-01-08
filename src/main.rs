use std::result::Result;
use sqlx::{migrate::MigrateDatabase, pool, sqlite::SqliteQueryResult, Sqlite, SqlitePool};

async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error>{
    let pool = SqlitePool::connect(&db_url).await?;
    let qry = r#"
    CREATE TABLE IS NOT EXISTS settings
    (
        settings_id     INTEGER PRIMARY KEY     NOT NULL,
        description     TEXT                    NOT NULL,
        created_on      DATETIME DEFAULT        (datetime('now', 'localtime')),
        updated_on      DATETIME DEFAULT        (datetime('now', 'localtime')),
        done            BOOLEAN                 NOT NULL DEFAULT 0
    );

    CREATE TABLE IS NOT EXISTS settings
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

}
