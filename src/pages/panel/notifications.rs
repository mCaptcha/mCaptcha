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
use actix_web::{HttpResponse, Responder};
use sailfish::TemplateOnce;

use crate::api::v1::notifications::get::{runner, NotificationResp};
use crate::errors::PageResult;
use crate::AppData;

#[derive(TemplateOnce)]
#[template(path = "panel/notifications/index.html")]
pub struct IndexPage {
    /// notifications
    n: Vec<NotificationResp>,
}

impl IndexPage {
    fn new(n: Vec<NotificationResp>) -> Self {
        IndexPage { n }
    }
}

const PAGE: &str = "Notifications";

#[my_codegen::get(path = "crate::PAGES.panel.notifications", wrap = "crate::CheckLogin")]
pub async fn notifications(data: AppData, id: Identity) -> PageResult<impl Responder> {
    let receiver = id.identity().unwrap();
    // TODO handle error where payload.to doesnt exist

    let notifications = runner::get_notification(&data, &receiver).await?;

    let body = IndexPage::new(notifications).render_once().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}
