use actix_web::{get, web, HttpResponse, Result};
use crate::api::{AppState, models::*};

#[get("/search")]
pub async fn search_stocks(
    data: web::Data<AppState>,
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse> {
    let mut query_params = query.into_inner();
    query_params.pagination.validate();
    
    let limit = query_params.pagination.limit;
    let offset = query_params.pagination.offset();
    let search_term = &query_params.q;
    
    match data.db.search_companies(search_term, limit, offset).await {
        Ok((companies, total)) => {
            let response_companies: Vec<SearchResultResponse> = companies
                .into_iter()
                .map(|company| SearchResultResponse {
                    symbol: company.symbol,
                    isin: company.isin,
                    series: company.series,
                    name: company.name,
                    latest_price: company.latest_price,
                    latest_date: company.latest_date.map(|d| d.format("%Y-%m-%d").to_string()),
                })
                .collect();
                
            let paginated = PaginatedResponse::new(
                response_companies, 
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
