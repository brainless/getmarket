use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: u32, limit: u32, total: u64) -> Self {
        let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
        let has_next = page < total_pages;
        let has_prev = page > 1;

        Self {
            data,
            pagination: PaginationMeta {
                page,
                limit,
                total,
                total_pages,
                has_next,
                has_prev,
            },
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 { 1 }
fn default_limit() -> u32 { 50 }

impl PaginationQuery {
    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.limit
    }
    
    pub fn validate(&mut self) {
        if self.page == 0 {
            self.page = 1;
        }
        if self.limit == 0 || self.limit > 1000 {
            self.limit = 50;
        }
    }
}
