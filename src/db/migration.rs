use std::error::Error;

use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");


pub fn run_migrations(connection: &mut SqliteConnection) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let pending_migrations = MigrationHarness::pending_migrations(connection, MIGRATIONS)?;
    
    if pending_migrations.is_empty() {
        return Ok(());
    }
    
    println!("Migrating...");

    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}