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

#![allow(clippy::type_complexity)]
use std::task::{Context, Poll};

use actix_identity::Identity;
//use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{http, Error, FromRequest, HttpResponse};

use futures::future::{ok, Either, Ready};

pub struct CheckLogin;

const LOGIN_ROUTE: &str = "/login";

impl<S, B> Transform<S> for CheckLogin
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckLoginMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckLoginMiddleware { service })
    }
}
//
//pub fn auto_login(req: &HttpRequest, pl: &mut dev::Payload) -> Option<()> {
//    dbg!("login");
//    if let Some(_) = Identity::from_request(req, pl)
//        .into_inner()
//        .map(|x| x.identity())
//        .unwrap()
//    {
//        Some(())
//    } else {
//        None
//    }
//}
//
//fn not_auth(path: &str) -> bool {
//    let paths = ["/login", "/css", "/img", "/js"];
//    paths.iter().any(|x| path.starts_with(x))
//}

pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S, B> Service for CheckLoginMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        //    if not_auth(req.path()) {
        //        return Either::Left(self.service.call(req));
        //    };
        let (r, mut pl) = req.into_parts();

        // TODO investigate when the bellow statement will
        // return error
        if let Ok(Some(_)) = Identity::from_request(&r, &mut pl)
            .into_inner()
            .map(|x| x.identity())
        {
            let req = ServiceRequest::from_parts(r, pl).ok().unwrap();
            Either::Left(self.service.call(req))
        //            Some(())
        } else {
            let req = ServiceRequest::from_parts(r, pl).ok().unwrap();
            Either::Right(ok(req.into_response(
                HttpResponse::Found()
                    .header(http::header::LOCATION, LOGIN_ROUTE)
                    .finish()
                    .into_body(),
            )))

            //None
        }

        //        let token = auto_login(&r, &mut pl);
        //        let req = ServiceRequest::from_parts(r, pl).ok().unwrap();
        //        if token.is_some() {
        //            Either::Left(self.service.call(req))
        //        } else {
        //            Either::Right(ok(req.into_response(
        //                HttpResponse::Found()
        //                    .header(http::header::LOCATION, LOGIN_ROUTE)
        //                    .finish()
        //                    .into_body(),
        //            )))
        //        }
    }
}
