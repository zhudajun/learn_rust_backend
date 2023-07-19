use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            //获取所有的电影
            .route("", web::get().to(get_all))
            // 获取特定的电影
            .route("/{film_id}", web::get().to(get))
            // 上传新的电影
            .route("", web::post().to(post))
            //更新电影
            .route("", web::put().to(put))
            // 删除电影
            .route("/{film_id}", web::delete().to(delet))
    );
}

/// 获取所有的电影
async fn get_all() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn post() -> HttpResponse {
    HttpResponse::Ok().finish()
}
async fn put() -> HttpResponse {
    HttpResponse::Ok().finish()
}
async fn delet() -> HttpResponse {
    HttpResponse::Ok().finish()
}
