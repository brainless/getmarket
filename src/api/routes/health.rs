use actix_web::{get, web, HttpResponse, Result};
use crate::api::{AppState, models::{ApiResponse, HealthResponse}};

#[get("/health")]
pub async fn health_check(data: web::Data<AppState>) -> Result<HttpResponse> {
    // Check database connectivity
    let db_status = match sqlx::query("SELECT 1").fetch_one(&data.db.pool).await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };
    
    let health = HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        database_status: db_status.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    
    let response = ApiResponse::success(health);
    Ok(HttpResponse::Ok().json(response))
}
