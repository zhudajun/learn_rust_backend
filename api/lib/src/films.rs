use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use shared::models::{CreateFilm, Film};
use uuid::Uuid;

use crate::film_repository::FilmRepository;


pub fn service<R: FilmRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            //获取所有的电影
            .route("", web::get().to(get_all::<R>))
            // 获取特定的电影
            .route("/{film_id}", web::get().to(get::<R>))
            // 上传新的电影
            .route("", web::post().to(post::<R>))
            //更新电影
            .route("", web::put().to(put::<R>))
            // 删除电影
            .route("/{film_id}", web::delete().to(delete::<R>)),
    );
}

/// 获取所有的电影
async fn get_all<R: FilmRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.get_films().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error :{:?}", e)),
    }
}

async fn get<R: FilmRepository>(film_id: web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
    match repo.get_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(_) => HttpResponse::NotFound().body("未发现"),
    }
}

async fn post<R: FilmRepository>(create_film:web::Json<CreateFilm>, repo: web::Data<R>) -> HttpResponse {
    match repo.create_film(&create_film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        },
    }
}
async fn put<R: FilmRepository>(film:web::Json<Film>, repo: web::Data<R>) -> HttpResponse {
    match repo.update_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e))
    }
}
async fn delete<R: FilmRepository>(film_id:web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
    match repo.delete_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}
