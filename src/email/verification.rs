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

use crate::AppData;
use crate::SETTINGS;

// The html we want to send.
const HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Hello from Lettre!</title>
</head>
<body>
    <div style="display: flex; flex-direction: column; align-items: center;">
        <h2 style="font-family: Arial, Helvetica, sans-serif;">Hello from Lettre!</h2>
        <h4 style="font-family: Arial, Helvetica, sans-serif;">A mailer library for Rust</h4>
    </div>
</body>
</html>"#;

async fn verification(data: &AppData) {
    if let Some(smtp) = SETTINGS.smtp.as_ref() {
        let from = format!("mCaptcha Admin <{}>", smtp.from);
        const SUBJECT: &str = "[mCaptcha] Please verify your email";

        let email = Message::builder()
            .from(from.parse().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to("Hei <hei@domain.tld>".parse().unwrap())
            .subject(SUBJECT)
            .multipart(
                MultiPart::alternative() // This is composed of two parts.
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(String::from(
                                "Hello from Lettre! A mailer library for Rust",
                            )), // Every message should have a plain text fallback.
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(String::from(HTML)),
                    ),
            )
            .unwrap();

        // unwrap is OK as SETTINGS.smtp is check at the start
        match data.mailer.as_ref().unwrap().send(email).await {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }
}
