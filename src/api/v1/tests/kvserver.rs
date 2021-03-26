/*
* Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use log::info;
use std::collections::HashMap;
use std::env;
use std::sync::mpsc;
use std::sync::{Arc, RwLock};

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

// from
// use crate::api::v1::mcaptcha::domains::Challenge;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Challenge {
    verification_challenge: String,
}

#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() {
    pretty_env_logger::init();
    let mut confif = env::args();
    confif.next();
    let port = confif.next().unwrap();
    HttpServer::new(move || {
        let store: UtilKVServer = Arc::new(RwLock::new(HashMap::new()));
        App::new()
            .data(store)
            .route("/{key}/", web::post().to(util_server_add))
            .route("/{key}/", web::get().to(util_server_retrive))
    })
    .bind(format!("localhost:{}", port))
    .unwrap()
    .run()
    .await
    .unwrap();
}

pub async fn server(ip: &str, tx: mpsc::Sender<Server>) {
    pretty_env_logger::init();
    let srv = HttpServer::new(move || {
        let store: UtilKVServer = Arc::new(RwLock::new(HashMap::new()));
        App::new()
            .data(store)
            .route("/{key}/", web::post().to(util_server_add))
            .route("/{key}/", web::get().to(util_server_retrive))
    })
    .bind(ip)
    .unwrap()
    .run();

    tx.send(srv.clone());
}

type UtilKVServer = Arc<RwLock<HashMap<String, Challenge>>>;

#[cfg(not(tarpaulin_include))]
async fn util_server_retrive(
    key: web::Path<String>,
    data: web::Data<UtilKVServer>,
) -> impl Responder {
    let key = key.into_inner();
    let store = data.read().unwrap();
    let resp = store.get(&key).unwrap();
    info!("key :{}, value: {:?}", key, resp);
    HttpResponse::Ok().json(resp)
}

#[cfg(not(tarpaulin_include))]
async fn util_server_add(
    key: web::Path<String>,
    payload: web::Json<Challenge>,
    data: web::Data<UtilKVServer>,
) -> impl Responder {
    info!("key :{}, value: {:?}", key, payload);
    let mut store = data.write().unwrap();
    store.insert(key.into_inner(), payload.into_inner());
    HttpResponse::Ok()
}
