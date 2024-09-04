use std::io;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};

// 라우트 구성
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// 핸들러 구성
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. EzyTutors is alive and kicking")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    //    app을 만들고 라우트를 구성한다.
    let app = move || App::new().configure(general_routes);

    //     HTTP 서버를 시작한다.
    HttpServer::new(app).bind("localhost:3000")?.run().await
}