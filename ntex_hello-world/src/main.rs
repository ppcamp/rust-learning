// This freaking ntex library already started with 8 cores (my maximum machine cores)

use ntex::web::{self, App, HttpRequest};

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

async fn test(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);

    "Hello world!"
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    // Env logger is needed for the middleware bellow
    env_logger::init();
    web::server(|| {
        println!("API is running in http://localhost:8080");

        App::new()
            // enable logger
            .case_insensitive_routing()
            .service((
                web::resource("/index").to(|| async { "Hello world!" }),
                web::resource("/").to(index),
                web::resource("/test").to(test),
            ))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
