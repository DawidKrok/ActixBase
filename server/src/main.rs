use actix_web::{HttpServer, App, web};
use actix_files::Files;
use listenfd::ListenFd;
use std::io;
use dotenv::dotenv;

use mongo;

mod config;
mod routes;


#[actix_web::main]
async fn main() -> io::Result<()> {
    // load variables from .env file
    dotenv().ok();
    let config = config::Config::from_env().unwrap();

    // web::Data will be used as App rescource and will share DB instance between all routes.
    let db = web::Data::new(
        mongo::DB::connect(&config.mongo.uri, &config.mongo.db).await.unwrap()
    );
    eprintln!("\x1b[42mConnected to MongoDB database \x1b[0m {}", config.mongo.db);
    
    // listenfd is used with cargo watch to reload API on every change
    // listenfd is responsible for keeping the Socket open while the project recompiles, so when it's ready, it will respond to waiting requests
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        App::new()
            // add App rescources (in this way every route can have access to DB instance)
            .app_data(db.clone())
            //=====| REGISTER STATIC FILES |=====
            .service(Files::new("/static", "./static").show_files_listing())
            //========| REGISTER ROUTES |========
            .configure(routes::config)
    });
    
    // if any TCP listeners were given, we use it (Server will run on Socket opened by systemfd). 
    // Otherwise server falls back to normal listening on host and port specified in .env file.
    // In this way we can opt out of using listenfd whenever we don't need it (e.g. when project is released)
    // use ` systemfd --no-pid -s http::8080 -- cargo watch -x run ` to run server on localhost:8080
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind((config.server.host, config.server.port))?,
    };

    eprintln!("\x1b[42mServer is running\x1b[0m");

    server.run()
    .await
}
