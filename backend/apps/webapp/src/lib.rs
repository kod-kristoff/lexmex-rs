use actix_web::{http, web, App, HttpServer};
use color_eyre::Result;

pub mod handlers;

pub async fn run() -> Result<()> {
    HttpServer::new(move || {
        App::new()
            .configure(handlers::web::init_routes)
    })
    .bind(format!("{}:{}", "0.0.0.0", 8000))?
    .run()
    .await?;

    Ok(())
}
