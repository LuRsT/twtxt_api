use actix_cors::Cors;
use actix_web::{web, Result};
use std::env;

async fn user(path: web::Path<(String,)>) -> Result<String> {
    let mut phrase: String = "Hello ".to_owned();
    let username: String = path.0.to_owned();

    phrase.push_str(&username);
    Ok(phrase)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::new().supports_credentials().finish())
            .service(web::resource("/user/{name}").route(web::get().to(user)))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
