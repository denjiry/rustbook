use actix_web::{HttpResponse, State};
use failure::Error;

use crate::Server;

pub fn handle_post_csv(server: State<Server>) -> Result<HttpResponse, Error> {
    unimplemented!()
}

pub fn handle_post_logs(server: State<Server>) -> Result<HttpResponse, Error> {
    unimplemented!()
}

pub fn handle_get_logs(server: State<Server>) -> Result<HttpResponse, Error> {
    unimplemented!()
}

pub fn handle_get_csv(server: State<Server>) -> Result<HttpResponse, Error> {
    unimplemented!()
}
