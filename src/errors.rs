use actix_files::NamedFile;
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, error, web, Error, HttpResponse, Result};

pub fn bad_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error = NamedFile::open("static/errors/400.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;

    Ok(ErrorHandlerResponse::Response(
        res.into_response(error.into_body()),
    ))
}

pub fn not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error = NamedFile::open("static/errors/404.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;

    Ok(ErrorHandlerResponse::Response(
        res.into_response(error.into_body()),
    ))
}

pub fn internal_server_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error = NamedFile::open("static/errors/500.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;

    Ok(ErrorHandlerResponse::Response(
        res.into_response(error.into_body()),
    ))
}
