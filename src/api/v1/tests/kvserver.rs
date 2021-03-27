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
use std::sync::mpsc;
use std::sync::{Arc, RwLock};

use actix_web::{dev::Server, middleware, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

/*
 * Simple KV Server that stores a json of with schema
 * `Challenge` at path /{key}/ on POST and emits on GET
 */

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Challenge {
    verification_challenge: String,
}

pub async fn server(ip: &str, tx: mpsc::Sender<Server>) {
    pretty_env_logger::init();
    let srv = HttpServer::new(move || {
        let store: UtilKVServer = Arc::new(RwLock::new(HashMap::new()));
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::default())
            .data(store)
            .route("/{key}/", web::post().to(util_server_add))
            .route("/{key}/", web::get().to(util_server_retrive))
    })
    .bind(ip)
    .unwrap()
    .run();

    tx.send(srv.clone()).unwrap();
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
    info!("[kv-server] retrive: key :{}, value: {:?}", key, resp);
    HttpResponse::Ok().json(resp)
}

#[cfg(not(tarpaulin_include))]
async fn util_server_add(
    key: web::Path<String>,
    payload: web::Json<Challenge>,
    data: web::Data<UtilKVServer>,
) -> impl Responder {
    info!("[kv-server] cache: key :{}, value: {:?}", key, payload);
    let mut store = data.write().unwrap();
    store.insert(key.into_inner(), payload.into_inner());
    HttpResponse::Ok()
}
