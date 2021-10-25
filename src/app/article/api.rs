use super::model::{Article, NewArticle, UpdateArticle};
use super::service;
use super::{request, response};
use crate::app::article::tag::model::{NewTag, Tag};
use crate::app::user::model::User;
use crate::utils::converter;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
// use diesel::Insertable;
use uuid::Uuid;

type ArticleIdSlug = Uuid;

pub async fn index() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("show_articles")
}

pub async fn feed() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("feed of articles")
}

pub async fn show() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("detail_article")
}

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<request::CreateArticleRequest>,
) -> Result<HttpResponse, HttpResponse> {
    let head = req.head();
    let extensions = head.extensions();
    let auth_user = extensions.get::<User>().expect("invalid auth user").clone();
    // --
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let new_slug = converter::to_kebab(&form.article.title);
    let (article, tag_list) = service::create(
        &conn,
        &NewArticle {
            author_id: auth_user.id,
            title: form.article.title.clone(),
            slug: new_slug,
            description: form.article.description.clone(),
            body: form.article.body.clone(),
        },
        &form.article.tagList,
    );
    let res = response::SingleArticleResponse::from(article, auth_user, tag_list);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
    form: web::Json<request::UpdateArticleRequest>,
) -> impl Responder {
    let head = req.head();
    let extensions = head.extensions();
    let auth_user = extensions.get::<User>().expect("invalid auth user").clone();
    // --
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    //
    let article_id = path.into_inner();

    {
        // TODO: move this logic into service
        use crate::schema::articles::dsl::*;
        use diesel::prelude::*;
        // TODO: validation deletable auth_user.id == article.author_id ?

        let new_slug = &form
            .article
            .title
            .as_ref()
            .map(|_title| converter::to_kebab(_title)); // TODO: move to static method in model
        diesel::update(articles.filter(id.eq(article_id)))
            .set(&UpdateArticle {
                // pub slug: Option<String>,
                slug: new_slug.to_owned(),
                title: form.article.title.clone(),
                description: form.article.description.clone(),
                body: form.article.body.clone(),
            })
            .get_result::<Article>(&conn)
            .expect("couldn't update article.");
    }
    HttpResponse::Ok().body("update_article")
}

pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
) -> impl Responder {
    let head = req.head();
    let extensions = head.extensions();
    let auth_user = extensions.get::<User>().expect("invalid auth user").clone();
    // --
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    //
    let article_id = path.into_inner();

    {
        // TODO: move this logic into service
        use crate::schema::articles::dsl::*;
        use diesel::prelude::*;

        // TODO: validation deletable auth_user.id == article.author_id ?

        diesel::delete(articles.filter(id.eq(article_id)))
            .execute(&conn)
            .expect("couldn't delete article by id.");
        // NOTE: references tag rows are deleted automatically by DELETE CASCADE
    }

    HttpResponse::Ok().json({})
}
