use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, test, web};

async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[test]
fn test_health_check() {
    let req = test::TestRequest::default().to_http_request();
    let resp = health_check(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
