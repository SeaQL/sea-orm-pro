use loco_rs::{cli, Result};
use migration::Migrator;
use sea_orm_pro_backend::app::App;

#[tokio::main]
#[allow(clippy::result_large_err)]
async fn main() -> Result<()> {
    // Load `.env`
    dotenvy::dotenv().ok();

    // Start the application
    cli::main::<App, Migrator>().await
}
