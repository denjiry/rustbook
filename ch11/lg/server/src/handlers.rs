use actix_web::{HttpResponse, Json, Query, State};
use failure::Error;
use log::debug;

use crate::Server;

pub fn handle_post_csv(server: State<Server>) -> Result<HttpResponse, Error> {
    let logs = Default::default();
    Ok(HttpResponse::Ok().json(api::csv::post::Response(logs)))
}

/// `POST /logs`のハンドラ
pub fn handle_post_logs(
    server: State<Server>,
    log: Json<api::logs::post::Request>,
) -> Result<HttpResponse, Error> {
    debug!("{:?}", log);
    Ok(HttpResponse::Accepted().finish())
}

/// `GET /logs`のハンドラ
pub fn handle_get_logs(
    server: State<Server>,
    range: Query<api::logs::get::Query>,
) -> Result<HttpResponse, Error> {
    debug!("{:?}", range);
    let logs = Default::default();
    Ok(HttpResponse::Ok().json(api::logs::get::Response(logs)))
}

/// `GET /csv`のハンドラ
pub fn handle_get_csv(
    server: State<Server>,
    range: Query<api::csv::get::Query>,
) -> Result<HttpResponse, Error> {
    debug!("{:?}", range);
    let csv: Vec<u8> = vec![];
    Ok(HttpResponse::Ok()
        .header("Content-type", "text.csv")
        .body(csv))
}