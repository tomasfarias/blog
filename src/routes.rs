use actix_web::{error, web, Error, HttpResponse, Result};
use actix_web::dev::{ServiceResponse, Body, ResponseBody};
use actix_web::http::{StatusCode, header};
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use tera::{Context, Tera};

use crate::db::{self, DBError};

pub async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut context = Context::new();

    let rendered = tmpl
        .render("index.html.tera", &context)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn blog(
    pool: web::Data<db::PgPool>,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    let posts = web::block(move || db::select_last_n_posts(10, &pool))
        .await?;
    context.insert("posts", &posts);

    let rendered = tmpl
        .render("blog.html.tera", &context)
        .map_err(|_| error::ErrorInternalServerError("An unexpected error has ocurred"))?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn post(
    pool: web::Data<db::PgPool>,
    tmpl: web::Data<Tera>,
    post_slug: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    let post = web::block(move || db::select_post_with_slug(&post_slug, &pool))
        .await?;
    context.insert("posts", &post);

    let rendered = tmpl
        .render("post.html.tera", &context)
        .map_err(|_| error::ErrorInternalServerError("An unexpected error has ocurred"))?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn hire_me(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut context = Context::new();

    let rendered = tmpl
        .render("hireme.html.tera", &context)
        .map_err(|_| error::ErrorInternalServerError("An unexpected error has ocurred"))?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub fn error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new()
        .handler(StatusCode::NOT_FOUND, not_found)
        .handler(StatusCode::BAD_REQUEST, bad_request)
        .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_server_error)
}

fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error_res = get_error_response(&res, "The resource could not be found");
    Ok(ErrorHandlerResponse::Response(error_res.unwrap_or(res)))
}

fn bad_request<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error_res = get_error_response(&res, "The request could not be processed");
    Ok(ErrorHandlerResponse::Response(error_res.unwrap_or(res)))
}

fn internal_server_error<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error_res = get_error_response(&res, "An unexpected error has ocurred");
    Ok(ErrorHandlerResponse::Response(error_res.unwrap_or(res)))
}

fn get_error_response<B>(res: &ServiceResponse<B>, message: &str) -> Option<ServiceResponse<B>> {
    let req = res.request();
    let tera = req.app_data::<web::Data<Tera>>().map(|t| t.get_ref());

    // Attempt to replace response body with template
    match tera {
        Some(tera) => {
            let mut context = tera::Context::new();
            context.insert("message", message);
            context.insert("reason", res.status().canonical_reason().unwrap_or("Error"));
            context.insert("status_code", res.status().as_str());
            let body = tera.render("error.html.tera", &context);

            match body {
                Ok(body) => {
                    let new_res = HttpResponse::build(res.status())
                        .set_header(header::CONTENT_TYPE, "text/html")
                        .finish();
                    let new_service_res = ServiceResponse::new(req.clone(), new_res)
                        .map_body(|_, _| ResponseBody::Body(Body::from(body)).into_body());
                    Some(new_service_res)
                },
                Err(_) => None,
            }
        },
        None => None,
    }
}
