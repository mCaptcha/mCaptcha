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
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{http, Error, FromRequest, HttpResponse};

use futures::future::{ok, Either, Ready};

use crate::PAGES;

pub struct CheckLogin;

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
        let (r, mut pl) = req.into_parts();

        // TODO investigate when the bellow statement will
        // return error
        if let Ok(Some(_)) = Identity::from_request(&r, &mut pl)
            .into_inner()
            .map(|x| x.identity())
        {
            let req = ServiceRequest::from_parts(r, pl).ok().unwrap();
            Either::Left(self.service.call(req))
        } else {
            let req = ServiceRequest::from_parts(r, pl).ok().unwrap();
            Either::Right(ok(req.into_response(
                HttpResponse::Found()
                    .header(http::header::LOCATION, PAGES.auth.login)
                    .finish()
                    .into_body(),
            )))
        }
    }
}
