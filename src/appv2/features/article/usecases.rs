use super::entities::Article;
use super::presenters::ArticlePresenter;
use super::repositories::{
    ArticleRepository, CreateArticleRepositoryInput, DeleteArticleRepositoryInput,
};
use super::services;
use crate::appv2::features::user::entities::User;
use crate::error::AppError;
use actix_web::HttpResponse;
use uuid::Uuid;

#[derive(Clone)]
pub struct ArticleUsecase {
    article_repository: ArticleRepository,
    article_presenter: ArticlePresenter,
}

impl ArticleUsecase {
    pub fn new(article_repository: ArticleRepository, article_presenter: ArticlePresenter) -> Self {
        Self {
            article_repository,
            article_presenter,
        }
    }

    pub fn fetch_articles_list(
        &self,
        params: services::FetchArticlesList,
    ) -> Result<HttpResponse, AppError> {
        let (list, count) =
            self.article_repository
                .fetch_articles_list(services::FetchArticlesList {
                    tag: params.tag.clone(),
                    author: params.author.clone(),
                    favorited: params.favorited.clone(),
                    offset: params.offset,
                    limit: params.limit,
                })?;
        let res = self.article_presenter.from_list_and_count(list, count);
        Ok(res)
    }

    pub fn fetch_article_by_slug(
        &self,
        params: &services::FetchArticleBySlug,
    ) -> Result<HttpResponse, AppError> {
        let article_title_slug = params.article_title_slug.clone();
        let result = self
            .article_repository
            .fetch_article_by_slug(article_title_slug)?;
        let res = self.article_presenter.from_item(result);
        Ok(res)
    }

    pub fn create(&self, params: CreateArticleUsecaseInput) -> Result<HttpResponse, AppError> {
        let slug = Article::convert_title_to_slug(&params.title);
        let result = self
            .article_repository
            .create(CreateArticleRepositoryInput {
                body: params.body,
                current_user: params.current_user,
                description: params.description,
                tag_name_list: params.tag_name_list,
                title: params.title,
                slug,
            })?;
        let res = self.article_presenter.from_item(result);
        Ok(res)
    }

    pub fn delete(&self, input: DeleteArticleUsercaseInput) -> Result<HttpResponse, AppError> {
        self.article_repository
            .delete(DeleteArticleRepositoryInput {
                slug: input.slug,
                author_id: input.author_id,
            })?;
        let res = self.article_presenter.toHttpRes();
        Ok(res)
    }
}

pub struct CreateArticleUsecaseInput {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_name_list: Option<Vec<String>>,
    pub current_user: User,
}

pub struct DeleteArticleUsercaseInput {
    pub slug: String,
    pub author_id: Uuid,
}
