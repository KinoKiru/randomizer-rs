use migration::sea_orm::{Database, DatabaseConnection};
use migration::{DbErr, Migrator, MigratorTrait};

// Connect to database and run migrations
pub async fn initialize(db_url: &String) -> Result<DatabaseConnection, DbErr> {
    info!("Trying to connect to database and run migrations");
    let conn = Database::connect(db_url).await?;
    Migrator::up(&conn, None).await?;
    info!("Connection made and migrations ran");
    Ok(conn)
}
