use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse,
};

pub const API_VERSION: &str = "v0.0.1";

#[get("/")]
async fn hello_world() -> &'static str {
    "hello world"
}
#[get("/version")]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
    tracing::info!("获取数据库版本");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;
    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}

async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Version", API_VERSION))
        .finish()
}

// 配置服务器
pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(version)
        .service(hello_world)
        .route("/health", web::get().to(health));
}


#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use super::*;
    #[actix_rt::test]
    async fn health_check_works() {
        let res = health().await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);
        let data = res.headers().get("Version").and_then(|h| h.to_str().ok());
        assert_eq!(data, Some(API_VERSION));
    }


}


