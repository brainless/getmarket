use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyResponse {
    pub id: i64,
    pub symbol: String,
    pub isin: String,
    pub series: String,
    pub name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockPriceResponse {
    pub id: i64,
    pub company_id: i64,
    pub symbol: String,
    pub trade_date: String,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub close_price: f64,
    pub last_price: f64,
    pub prev_close: f64,
    pub total_traded_qty: i64,
    pub total_traded_value: f64,
    pub total_trades: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub database_status: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketOverviewResponse {
    pub total_companies: i64,
    pub total_price_records: i64,
    pub latest_trading_date: Option<String>,
    pub top_gainers: Vec<TopPerformerResponse>,
    pub top_losers: Vec<TopPerformerResponse>,
    pub most_active: Vec<TopPerformerResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopPerformerResponse {
    pub symbol: String,
    pub series: String,
    pub latest_price: f64,
    pub prev_close: f64,
    pub price_change: f64,
    pub price_change_percent: f64,
    pub volume: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResultResponse {
    pub symbol: String,
    pub isin: String,
    pub series: String,
    pub name: Option<String>,
    pub latest_price: Option<f64>,
    pub latest_date: Option<String>,
}
