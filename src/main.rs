use actix_files::NamedFile;
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use env_logger::Env;
use std::path::PathBuf;

async fn index() -> Result<NamedFile> {
    let path: PathBuf = PathBuf::from("./public/index.html");
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

// For testing
async fn _ping(req: HttpRequest) -> HttpResponse {
    match req.headers().get("content-type") {
        Some(_) => HttpResponse::Ok()
            .content_type("plain/text")
            .body("running"),
        None => HttpResponse::BadRequest().body("error"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};

    #[actix_rt::test]
    async fn test_index_ok() {
        let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
        let resp = _ping(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_index_not_ok() {
        let req = test::TestRequest::default().to_http_request();
        let resp = _ping(req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }
}
