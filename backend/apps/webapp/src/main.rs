use color_eyre::Result;
use eyre::ErrReport;

use lexmex_webapp::run;


#[actix_web::main]
async fn main() -> Result<()> {
    // Load configuration
    // ------------------
    // let settings = Config::from_env()?;

    // Install Color Eyre
    // ------------------
    color_eyre::install()?;

    // Initialization Postgres Pool
    // ----------------------------
    // let db_pool = configure_with_db_url(&settings.database_url).await?;

    // Runs migrations
    // ---------------
    // if settings.database_auto_migration {
    //     sqlx::migrate!("./../../migrations").run(&db_pool).await?;
    // }

    // run(settings, db_pool).await
    run().await
}
