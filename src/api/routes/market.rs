use actix_web::{get, web, HttpResponse, Result};
use crate::api::{AppState, models::*};

#[get("/market/overview")]
pub async fn get_market_overview(
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    match data.db.get_market_overview().await {
        Ok(market_data) => {
            let response = MarketOverviewResponse {
                total_companies: market_data.total_companies,
                total_price_records: market_data.total_price_records,
                latest_trading_date: market_data.latest_trading_date
                    .map(|d| d.format("%Y-%m-%d").to_string()),
                top_gainers: market_data.top_gainers
                    .into_iter()
                    .map(|p| TopPerformerResponse {
                        symbol: p.symbol,
                        series: p.series,
                        latest_price: p.latest_price,
                        prev_close: p.prev_close,
                        price_change: p.price_change,
                        price_change_percent: p.price_change_percent,
                        volume: p.volume,
                    })
                    .collect(),
                top_losers: market_data.top_losers
                    .into_iter()
                    .map(|p| TopPerformerResponse {
                        symbol: p.symbol,
                        series: p.series,
                        latest_price: p.latest_price,
                        prev_close: p.prev_close,
                        price_change: p.price_change,
                        price_change_percent: p.price_change_percent,
                        volume: p.volume,
                    })
                    .collect(),
                most_active: market_data.most_active
                    .into_iter()
                    .map(|p| TopPerformerResponse {
                        symbol: p.symbol,
                        series: p.series,
                        latest_price: p.latest_price,
                        prev_close: p.prev_close,
                        price_change: p.price_change,
                        price_change_percent: p.price_change_percent,
                        volume: p.volume,
                    })
                    .collect(),
            };
            
            let api_response = ApiResponse::success(response);
            Ok(HttpResponse::Ok().json(api_response))
        },
        Err(e) => {
            let response = ApiResponse::<()>::error(format!("Database error: {}", e));
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}
