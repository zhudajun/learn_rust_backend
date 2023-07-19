use super::{FilmRepository, FilmResult};
use async_trait::async_trait;
use chrono::Utc;
use shared::models::{CreateFilm, Film};
use std::{collections::HashMap, sync::RwLock};
use uuid::Uuid;

pub struct MemoryFilmRepository {
    films: RwLock<HashMap<Uuid, Film>>,
}

impl MemoryFilmRepository {
    pub fn new() -> Self {
        Self {
            films: RwLock::new(HashMap::new()),
        }
    }
}
impl Default for MemoryFilmRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FilmRepository for MemoryFilmRepository {
    async fn get_films(&self) -> FilmResult<Vec<Film>> {
        let result = self
            .films
            .read()
            .map(|films| films.clone().into_values().collect::<Vec<_>>())
            .map_err(|e| format!("读错误 :{}", e));
        if result.is_err() {
            tracing::error!("不能读取 电影");
        }
        result
    }
    async fn get_film(&self, film_id: &uuid::Uuid) -> FilmResult<Film> {
        let result = self
            .films
            .read()
            .map_err(|e| format!("读取单个影片错误 {}", e))
            .and_then(|films| {
                films
                    .get(film_id)
                    .cloned()
                    .ok_or_else(|| format!("找不到这个电影 {}", film_id))
            });
        if result.is_err() {
            tracing::error!("找不到这个电影,电影的 ID 是 {}", film_id);
        }
        result
    }
    async fn create_film(&self, create_film: &CreateFilm) -> FilmResult<Film> {
        match self.films.write() {
            Ok(mut films) => {
                let new_film = Film {
                    id: uuid::Uuid::new_v4(),
                    title: create_film.title.clone(),
                    director: create_film.director.clone(),
                    year: create_film.year,
                    poster: create_film.poster.clone(),
                    created_at: Some(Utc::now()),
                    updated_at: None,
                };
                films.insert(new_film.id, new_film.clone());
                tracing::trace!("电影创建成功 {}", new_film.id);
                Ok(new_film)
            }
            Err(e) => {
                let err = format!("添加电影的时候出现错误 {}", e);
                tracing::error!(err);
                Err(err)
            }
        }

    }
    async fn update_film(&self, film:&Film) -> FilmResult<Film> {
        match self.films.write() {
            Ok(mut films) => {
                let old_film = films.get_mut(&film.id);
                match old_film {
                    Some(old_film) => {
                        let mut updated_film = film.to_owned();
                        updated_film.created_at = old_film.created_at;
                        updated_film.updated_at = Some(Utc::now());
                        films.insert(film.id, updated_film.clone());
                        tracing::debug!("电影 {} 已经更新了", film.id);
                        Ok(updated_film)
                    }
                    None => {
                        let err = format!("不能更新电影,电影不存在 {}", film.id);
                        tracing::error!(err);
                        Err(err)
                    }
                }
            }
            Err(e) => {
                let err = format!("更新电影的过程中发生错误 {}",e);
                tracing::error!(err);
                Err(err)
            }
        }
    }

    async fn delete_film(&self, film_id: &uuid::Uuid) -> FilmResult<Uuid> {
        match self.films.write() {
            Ok(mut films) => {
                films.remove(film_id);
                Ok(film_id.to_owned())
            }
            Err(e) => {
                let err = format!("删除电影的时候发生错误 {}", e);
                tracing::error!(err);
                Err(err)
            }
        }
    }
}
