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
//! Email operations: verification, notification, etc
use lettre::{
    message::{header, MultiPart, SinglePart},
    AsyncTransport, Message,
};
use sailfish::TemplateOnce;

use crate::errors::*;
use crate::Data;
use crate::SETTINGS;

const PAGE: &str = "Login";

#[derive(Clone, TemplateOnce)]
#[template(path = "email/verification/index.html")]
struct IndexPage<'a> {
    verification_link: &'a str,
}

impl<'a> IndexPage<'a> {
    fn new(verification_link: &'a str) -> Self {
        Self { verification_link }
    }
}

async fn verification(
    data: &Data,
    to: &str,
    verification_link: &str,
) -> ServiceResult<()> {
    if let Some(smtp) = SETTINGS.smtp.as_ref() {
        let from = format!("mCaptcha Admin <{}>", smtp.from);
        let reply_to = format!("mCaptcha Admin <{}>", smtp.reply_to);
        const SUBJECT: &str = "[mCaptcha] Please verify your email";

        let plain_text = format!(
            "
Welcome to mCaptcha!

Please verify your email address to continue.

VERIFICATION LINK: {}

Please ignore this email if you weren't expecting it.

With best regards,
Admin
instance: {}
project website: {}",
            verification_link,
            SETTINGS.server.domain,
            crate::PKG_HOMEPAGE
        );

        let html = IndexPage::new(verification_link).render_once().unwrap();

        let email = Message::builder()
            .from(from.parse().unwrap())
            .reply_to(reply_to.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(SUBJECT)
            .multipart(
                MultiPart::alternative() // This is composed of two parts.
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(plain_text), // Every message should have a plain text fallback.
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(html),
                    ),
            )
            .unwrap();

        // unwrap is OK as SETTINGS.smtp is check at the start
        data.mailer.as_ref().unwrap().send(email).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use awc::Client;

    #[actix_rt::test]
    async fn email_verification_works() {
        const TO_ADDR: &str = "Hello <realaravinth@localhost>";
        const VERIFICATION_LINK: &str = "https://localhost";
        let data = Data::new().await;
        verification(&data, TO_ADDR, VERIFICATION_LINK)
            .await
            .unwrap();

        let client = Client::default();
        let mut resp = client.get("http://localhost:1080/email")
            .send()
            .await
            .unwrap();
        let data: serde_json::Value = resp.json().await.unwrap();
        let data = &data[0];
        let smtp = SETTINGS.smtp.as_ref().unwrap();

        let from_addr = &data["headers"]["from"];["address"];

        assert!(from_addr.to_string().contains(&smtp.from));

        let body = &data["html"];
        assert!(body.to_string().contains(VERIFICATION_LINK));
    }
}
