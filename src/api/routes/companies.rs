use actix_web::{get, web, HttpResponse, Result};
use crate::api::{AppState, models::*};

#[get("/companies")]
pub async fn get_companies(
    data: web::Data<AppState>,
    query: web::Query<CompaniesQuery>,
) -> Result<HttpResponse> {
    let mut query_params = query.into_inner();
    query_params.pagination.validate();
    
    let limit = query_params.pagination.limit;
    let offset = query_params.pagination.offset();
    let search = query_params.search.as_deref();
    let series = query_params.series.as_deref();
    
    match data.db.get_companies(limit, offset, search, series).await {
        Ok((companies, total)) => {
            let response_companies: Vec<CompanyResponse> = companies
                .into_iter()
                .map(|company| CompanyResponse {
                    id: company.id,
                    symbol: company.symbol,
                    isin: company.isin,
                    series: company.series,
                    name: company.name,
                    created_at: company.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
                    updated_at: company.updated_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
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
