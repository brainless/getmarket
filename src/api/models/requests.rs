use serde::Deserialize;
use super::PaginationQuery;

#[derive(Debug, Deserialize)]
pub struct CompaniesQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,
    pub search: Option<String>,
    pub series: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StockPricesQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LatestPricesQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,
    pub date: Option<String>,
    pub series: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,
    pub q: String,
}

#[derive(Debug, Deserialize)]
pub struct TopPerformersQuery {
    #[serde(default = "default_limit_10")]
    pub limit: u32,
    pub period: Option<String>,
    pub metric: Option<String>,
}

fn default_limit_10() -> u32 { 10 }

impl TopPerformersQuery {
    pub fn validate(&mut self) {
        if self.limit == 0 || self.limit > 100 {
            self.limit = 10;
        }
    }
}
