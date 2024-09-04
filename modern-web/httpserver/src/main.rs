use server::Server;

mod handler;
mod router;
mod server;

fn main() {
    // 서버를 시작한다
    let server = Server::new("127.0.0.1:3000");
    // 서버를 실행한다
    server.run();
}