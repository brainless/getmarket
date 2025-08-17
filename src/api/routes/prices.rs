use actix_web::{get, web, HttpResponse, Result};
use crate::api::{AppState, models::*};

#[get("/companies/{symbol}/prices")]
pub async fn get_company_prices(
    data: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<StockPricesQuery>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let mut query_params = query.into_inner();
    query_params.pagination.validate();
    
    let limit = query_params.pagination.limit;
    let offset = query_params.pagination.offset();
    let from_date = query_params.from_date.as_deref();
    let to_date = query_params.to_date.as_deref();
    
    match data.db.get_company_prices(&symbol, limit, offset, from_date, to_date).await {
        Ok((prices, total)) => {
            let response_prices: Vec<StockPriceResponse> = prices
                .into_iter()
                .map(|price| StockPriceResponse {
                    id: price.id,
                    company_id: price.company_id,
                    symbol: price.symbol,
                    trade_date: price.trade_date.format("%Y-%m-%d").to_string(),
                    open_price: price.open_price,
                    high_price: price.high_price,
                    low_price: price.low_price,
                    close_price: price.close_price,
                    last_price: price.last_price,
                    prev_close: price.prev_close,
                    total_traded_qty: price.total_traded_qty,
                    total_traded_value: price.total_traded_value,
                    total_trades: price.total_trades,
                    created_at: price.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
                })
                .collect();
                
            let paginated = PaginatedResponse::new(
                response_prices, 
                query_params.pagination.page, 
                limit, 
                total
            );
            
            let response = ApiResponse::success(paginated);
            Ok(HttpResponse::Ok().json(response))
        },
        Err(e) => {
            let response = ApiResponse::<()>::error(format!("Database error: {}", e));
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

#[get("/prices/latest")]
pub async fn get_latest_prices(
    data: web::Data<AppState>,
    query: web::Query<LatestPricesQuery>,
) -> Result<HttpResponse> {
    let mut query_params = query.into_inner();
    query_params.pagination.validate();
    
    let limit = query_params.pagination.limit;
    let offset = query_params.pagination.offset();
    let date = query_params.date.as_deref();
    let series = query_params.series.as_deref();
    
    match data.db.get_latest_prices(limit, offset, date, series).await {
        Ok((prices, total)) => {
            let response_prices: Vec<StockPriceResponse> = prices
                .into_iter()
                .map(|price| StockPriceResponse {
                    id: price.id,
                    company_id: price.company_id,
                    symbol: price.symbol,
                    trade_date: price.trade_date.format("%Y-%m-%d").to_string(),
                    open_price: price.open_price,
                    high_price: price.high_price,
                    low_price: price.low_price,
                    close_price: price.close_price,
                    last_price: price.last_price,
                    prev_close: price.prev_close,
                    total_traded_qty: price.total_traded_qty,
                    total_traded_value: price.total_traded_value,
                    total_trades: price.total_trades,
                    created_at: price.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
                })
                .collect();
                
            let paginated = PaginatedResponse::new(
                response_prices, 
                query_params.pagination.page, 
                limit, 
                total
            );
            
            let response = ApiResponse::success(paginated);
            Ok(HttpResponse::Ok().json(response))
        },
        Err(e) => {
            let response = ApiResponse::<()>::error(format!("Database error: {}", e));
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}
