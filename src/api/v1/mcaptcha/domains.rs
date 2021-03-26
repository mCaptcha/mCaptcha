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

use actix_identity::Identity;
use actix_web::{post, web, HttpResponse, Responder};
use awc::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use super::{get_random, is_authenticated};
use crate::errors::*;
use crate::Data;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Domain {
    pub name: String,
}

#[post("/api/v1/mcaptcha/domain/add")]
pub async fn add_domain(
    payload: web::Json<Domain>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;
    let url = Url::parse(&payload.name)?;

    let host = url.host_str().ok_or(ServiceError::NotAUrl)?;
    let user = id.identity().unwrap();
    let challenge = get_random(32);
    let res = sqlx::query!(
        "INSERT INTO mcaptcha_domains_unverified (name, owner_id, verification_challenge) VALUES  
            ($1, (SELECT ID FROM mcaptcha_users WHERE name = ($2) ), $3);",
        host,
        user,
        challenge
    )
    .execute(&data.db)
    .await;
    match res {
        Err(e) => Err(dup_error(e, ServiceError::HostnameTaken)),
        Ok(_) => Ok(HttpResponse::Ok()),
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Challenge {
    verification_challenge: String,
}

#[post("/api/v1/mcaptcha/domain/verify/challenge/get")]
pub async fn get_challenge(
    payload: web::Json<Domain>,
    data: web::Data<Data>,
    id: Identity,
    client: web::Data<Client>,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;
    let url = Url::parse(&payload.name)?;

    let host = url.host_str().ok_or(ServiceError::NotAUrl).unwrap();
    let user = id.identity().unwrap();
    let res = sqlx::query_as!(
        Challenge,
        "SELECT verification_challenge 
            FROM mcaptcha_domains_unverified where 
            name = $1 AND owner_id = (SELECT ID from mcaptcha_users where name = $2)",
        host,
        user,
    )
    .fetch_one(&data.db)
    .await
    .unwrap();
    Ok(HttpResponse::Ok().json(res))
}

#[post("/api/v1/mcaptcha/domain/verify/challenge/prove")]
pub async fn verify(
    payload: web::Json<Domain>,
    data: web::Data<Data>,
    client: web::Data<Client>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    use futures::{future::TryFutureExt, try_join};

    is_authenticated(&id).unwrap();
    //let url = Url::parse(&payload.name).unwrap();
    //let host = url.host_str().ok_or(ServiceError::NotAUrl).unwrap();
    //let user = id.identity().unwrap();
    //let challenge_fut = sqlx::query_as!(
    //    Challenge,
    //    "SELECT verification_challenge
    //        FROM mcaptcha_domains_unverified where
    //        name = $1 AND owner_id = (SELECT ID from mcaptcha_users where name = $2)",
    //    &host,
    //    &user,
    //)
    //.fetch_one(&data.db)
    //.map_err(|e| {
    //    let r: ServiceError = e.into();
    //    r
    //});

    //let res_fut = client.get(host).send().map_err(|e| {
    //    let r: ServiceError = e.into();
    //    r
    //});

    //let (challenge, mut server_res) = try_join!(challenge_fut, res_fut).unwrap();

    //let server_resp: Challenge = server_res
    //    .json()
    //    .await
    //    .map_err(|_| return ServiceError::ChallengeCourruption)
    //    .unwrap();

    //if server_resp.verification_challenge == challenge.verification_challenge {
    //    sqlx::query!(
    //        "INSERT INTO mcaptcha_domains_verified (name, owner_id) VALUES
    //        ($1, (SELECT ID from mcaptcha_users WHERE name = $2))",
    //        &host,
    //        &user
    //    )
    //    .execute(&data.db)
    //    .await
    //    .unwrap();

    //    // TODO delete staging unverified

    Ok(HttpResponse::Ok())
    //} else {
    //    Err(ServiceError::ChallengeVerificationFailure)
    //}
}

#[post("/api/v1/mcaptcha/domain/delete")]
pub async fn delete_domain(
    payload: web::Json<Domain>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;
    let url = Url::parse(&payload.name)?;
    let host = url.host_str().ok_or(ServiceError::NotAUrl)?;
    sqlx::query!(
        "DELETE FROM mcaptcha_domains_verified WHERE name = ($1)",
        host,
    )
    .execute(&data.db)
    .await?;
    Ok(HttpResponse::Ok())
}

// Workflow:
// 1. Sign up
// 2. Sign in
// 3. Add domain(DNS TXT record verification? / put string at path)
// 4. Create token
// 5. Add levels
// 6. Update duration
// 7. Start syatem

#[cfg(test)]
mod tests {
    use actix_web::http::{header, StatusCode};
    use actix_web::test;

    use super::*;
    use crate::api::v1::services as v1_services;
    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn add_domains_work() {
        const NAME: &str = "testuserdomainn";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL: &str = "testuserdomain@a.com";
        const DOMAIN: &str = "http://example.com";
        const ADD_URL: &str = "/api/v1/mcaptcha/domain/add";

        {
            let data = Data::new().await;
            delete_user(NAME, &data).await;
        }

        register_and_signin(NAME, EMAIL, PASSWORD).await;

        // 1. add domain
        let (data, _, signin_resp) = add_domain_util(NAME, PASSWORD, DOMAIN).await;
        let cookies = get_cookie!(signin_resp);
        let mut app = get_app!(data).await;

        let mut domain = Domain {
            name: DOMAIN.into(),
        };

        // 2. duplicate domain
        bad_post_req_test(
            NAME,
            PASSWORD,
            ADD_URL,
            &domain,
            ServiceError::HostnameTaken,
            StatusCode::BAD_REQUEST,
        )
        .await;

        // 3. delete domain
        let del_domain_resp = test::call_service(
            &mut app,
            post_request!(&domain, "/api/v1/mcaptcha/domain/delete")
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(del_domain_resp.status(), StatusCode::OK);

        // 4. not a URL test for adding domain
        domain.name = "testing".into();
        bad_post_req_test(
            NAME,
            PASSWORD,
            ADD_URL,
            &domain,
            ServiceError::NotAUrl,
            StatusCode::BAD_REQUEST,
        )
        .await;
    }

    #[actix_rt::test]
    async fn domain_verification_works() {
        use crate::api::v1::tests::*;
        use awc::Client;
        use std::sync::mpsc;
        use std::thread;

        const NAME: &str = "testdomainveri";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL: &str = "domainverification@a.com";
        const DOMAIN: &str = "http://localhost:18001";
        const IP: &str = "localhost:18001";
        const CHALLENGE_GET: &str = "/api/v1/mcaptcha/domain/verify/challenge/get";
        const CHALLENGE_VERIFY: &str = "/api/v1/mcaptcha/domain/verify/challenge/prove";
        {
            let data = Data::new().await;
            delete_user(NAME, &data).await;
        }

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            rt::System::new("").block_on(server(IP, tx));
        });
        let srv = rx.recv().unwrap();

        let client = Client::new();

        let (data, _, signin_resp) = register_and_signin(NAME, EMAIL, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let mut app = test::init_service(
            App::new()
                .wrap(get_identity_service())
                .configure(v1_services)
                .data(data.clone())
                .data(client.clone()),
        )
        .await;

        let domain = Domain {
            name: DOMAIN.into(),
        };

        let add_domain_resp = test::call_service(
            &mut app,
            post_request!(&domain, "/api/v1/mcaptcha/domain/add")
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(add_domain_resp.status(), StatusCode::OK);

        let get_challenge_resp = test::call_service(
            &mut app,
            post_request!(&domain, CHALLENGE_GET)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(get_challenge_resp.status(), StatusCode::OK);
        let challenge: Challenge = test::read_body_json(get_challenge_resp).await;

        client
            .post(format!("{}/domain_verification_works/", DOMAIN))
            .send_json(&challenge)
            .await
            .unwrap();

        let verify_challenge_resp = test::call_service(
            &mut app,
            post_request!(&domain, CHALLENGE_VERIFY)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(verify_challenge_resp.status(), StatusCode::OK);
        srv.stop(true).await;
    }
}
