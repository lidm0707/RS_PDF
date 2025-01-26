use diesel::{prelude::SqliteConnection, Connection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// use dotenvy::dotenv;
// use std::env;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./src/backend/repo/migrations");
const DATABASE: &str = "database.db";

pub fn connect_database() -> SqliteConnection {
    // dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // SqliteConnection::establish(&database_url)
    //     .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    SqliteConnection::establish(DATABASE)
    .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE))
}


pub fn run_migrations(conn: &mut SqliteConnection) -> Result<(), anyhow::Error> {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
    Ok(())
}
