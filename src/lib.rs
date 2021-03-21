use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use std::net::TcpListener;

// Initially returned `impl Responder`.
// Now explicitly state the type from `actix-web`.
// No performance difference. Just a stylistic choice.
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Let's start simple: we always return a 200 OK.
async fn subscribe(_form: web::Form<SubscriberData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct SubscriberData {
    email: String,
    name: String,
}

#[test]
fn test_health_check() {
    let req = test::TestRequest::default().to_http_request();
    let resp = health_check(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // // Retrieve the port assigned by the OS
    // let port = listener.local_addr().unwrap().port();
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            // A new entry in our routing table for POST /subscriptions requests
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    // No .await here
    Ok(server)
}
