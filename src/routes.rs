use actix_web::web;
use crate::handlers::{upload, parse};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(upload::upload)  // Register the upload route
       .service(parse::parse);    // Register the parse route
}
