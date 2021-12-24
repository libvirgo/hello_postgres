use futures::TryStreamExt;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:abc123@127.0.0.1:55433/postgres")
        .await?;
    let mut rows = sqlx::query("SELECT tablename FROM pg_tables WHERE schemaname = $1")
        .bind("public")
        .fetch(&pool);
    while let Some(row) = rows.try_next().await? {
        let tb_name: &str = row.try_get("tablename")?;
        println!("{}", tb_name)
    }
    Ok(())
}
