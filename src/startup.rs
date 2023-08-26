use std::net::TcpListener;

use crate::prisma::PrismaClient;
use crate::routes::{health_check, subscribe};
use actix_web::middleware::Logger;
use actix_web::{dev::Server, web, App, HttpServer};

pub fn run(listener: TcpListener, prisma_client: PrismaClient) -> Result<Server, std::io::Error> {
    let prisma_client = web::Data::new(prisma_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(prisma_client.clone())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
