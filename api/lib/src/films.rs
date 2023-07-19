use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use shared::models::{CreateFilm, Film};
use uuid::Uuid;

use crate::film_repository::FilmRepository;
type Repository = web::Data<Box<dyn FilmRepository>>;

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
            .route("/{film_id}", web::delete().to(delet)),
    );
}

/// 获取所有的电影
async fn get_all(repo: Repository) -> HttpResponse {
    match repo.get_films().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error :{:?}", e)),
    }
}

async fn get(film_id: web::Path<Uuid>, repo: Repository) -> HttpResponse {
    match repo.get_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(_) => HttpResponse::NotFound().body("未发现"),
    }
}

async fn post(create_film:web::Json<CreateFilm>, repo: Repository) -> HttpResponse {
    match repo.create_film(&create_film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        },
    }
}
async fn put(film:web::Json<Film>, repo: Repository) -> HttpResponse {
    match repo.update_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e))
    }
}
async fn delet(film_id:web::Path<Uuid>, repo: Repository) -> HttpResponse {
    match repo.delete_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}
