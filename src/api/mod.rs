pub mod routes;
pub mod models;
pub mod middleware;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use crate::database::Database;

pub struct AppState {
    pub db: Database,
}

pub async fn start_server(host: String, port: u16, db: Database) -> std::io::Result<()> {
    println!("🚀 Starting API server at http://{}:{}", host, port);
    
    HttpServer::new(move || {
        let app_state = AppState { db: db.clone() };
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            )
            .service(
                web::scope("/api")
                    .service(routes::health::health_check)
                    .service(routes::companies::get_companies)
                    .service(routes::prices::get_company_prices)
                    .service(routes::prices::get_latest_prices)
                    .service(routes::search::search_stocks)
                    .service(routes::market::get_market_overview)
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
