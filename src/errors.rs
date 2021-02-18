use actix_web::{web, HttpResponse, Result};
use actix_web::dev::{ServiceResponse, Body, ResponseBody};
use actix_web::http::{StatusCode, header};
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use tera::{Context, Tera};

pub fn init_error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new()
        .handler(StatusCode::NOT_FOUND, not_found)
        .handler(StatusCode::BAD_REQUEST, bad_request)
        .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_server_error)
        .handler(StatusCode::FORBIDDEN, forbidden)
}

fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error_res = get_error_response(&res, "The resource could not be found.");
    Ok(ErrorHandlerResponse::Response(error_res.unwrap_or(res)))
}

fn bad_request<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error_res = get_error_response(&res, "The request could not be processed.");
    Ok(ErrorHandlerResponse::Response(error_res.unwrap_or(res)))
}

fn internal_server_error<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error_res = get_error_response(&res, "An unexpected error has ocurred. Try gain later.");
    Ok(ErrorHandlerResponse::Response(error_res.unwrap_or(res)))
}

fn forbidden<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error_res = get_error_response(&res, "You are not authorized to perform this request.");
    Ok(ErrorHandlerResponse::Response(error_res.unwrap_or(res)))
}

fn get_error_response<B>(res: &ServiceResponse<B>, message: &str) -> Option<ServiceResponse<B>> {
    let req = res.request();
    let tera = req.app_data::<web::Data<Tera>>().map(|t| t.get_ref());

    // Attempt to replace response body with template
    match tera {
        Some(tera) => {
            let mut context = Context::new();

            // Override generic message with error message, if available
            let new_message = match res.response().error() {
                Some(e) => e.to_string(),
                None => message.to_string(),
            };

            context.insert("message", &new_message);
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
