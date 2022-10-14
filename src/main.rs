#[macro_use]
extern crate actix_web;
use std::{env,io};
use actix_web::{middleware,App,HTTPServer};

mod constants;
mod like;
mod response;
mod tweet;

#[actix_rt::main]
async fn main()-> io::Result<()> {
    env:set_var("Rust_Log", "actix_web=debug",actix_server=info);
    env_logger::init();

    HTTPServer::new(||{

        App::new()
                //enable logger - always register actix-web Logger middleware last
             .wrap(middleware::Logger::default())
             .service(tweet::list)
             .service(tweet::get)
             .service(tweet::create)
             .service(tweet::delete)
             .service(like::list)
             .service(like::plus_one)
             .service(like::minus_one)
    }).bind("0.0.0.0:9090")?.run().await
        
}
