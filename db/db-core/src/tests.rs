/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
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
//! Test utilities
use crate::errors::*;
use crate::prelude::*;

/// test all database functions
pub async fn database_works<'a, T: MCDatabase>(
    db: &T,
    p: &Register<'a>,
    c: &CreateCaptcha<'a>,
    l: &[Level],
    tp: &TrafficPattern,
    an: &AddNotification<'a>,
) {
    assert!(db.ping().await, "ping test");
    if db.username_exists(p.username).await.unwrap() {
        db.delete_user(p.username).await.unwrap();
        assert!(
            !db.username_exists(p.username).await.unwrap(),
            "user is deleted so username shouldn't exsit"
        );
    }
    db.register(p).await.unwrap();

    // testing get secret
    let secret = db.get_secret(p.username).await.unwrap();
    assert_eq!(secret.secret, p.secret, "user secret matches");

    // testing update secret: setting secret = username
    db.update_secret(p.username, p.username).await.unwrap();
    let secret = db.get_secret(p.username).await.unwrap();
    assert_eq!(
        secret.secret, p.username,
        "user secret matches username; as set by previous step"
    );

    // testing get_password

    // with username
    let name_hash = db.get_password(&Login::Username(p.username)).await.unwrap();
    assert_eq!(name_hash.hash, p.hash, "user password matches");

    assert_eq!(name_hash.username, p.username, "username matches");

    // with email
    let mut name_hash = db
        .get_password(&Login::Email(p.email.as_ref().unwrap()))
        .await
        .unwrap();
    assert_eq!(name_hash.hash, p.hash, "user password matches");
    assert_eq!(name_hash.username, p.username, "username matches");

    // testing email exists
    assert!(
        db.email_exists(p.email.as_ref().unwrap()).await.unwrap(),
        "user is registered so email should exsit"
    );
    assert!(
        db.username_exists(p.username).await.unwrap(),
        "user is registered so username should exsit"
    );

    // update password test. setting password = username
    name_hash.hash = name_hash.username.clone();
    db.update_password(&name_hash).await.unwrap();

    let name_hash = db.get_password(&Login::Username(p.username)).await.unwrap();
    assert_eq!(
        name_hash.hash, p.username,
        "user password matches with changed value"
    );
    assert_eq!(name_hash.username, p.username, "username matches");

    // update username to p.email
    assert!(
        !db.username_exists(p.email.as_ref().unwrap()).await.unwrap(),
        "user with p.email doesn't exist. pre-check to update username to p.email"
    );
    db.update_username(p.username, p.email.as_ref().unwrap())
        .await
        .unwrap();
    assert!(
        db.username_exists(p.email.as_ref().unwrap()).await.unwrap(),
        "user with p.email exist post-update"
    );

    // deleting user for re-registration with email = None
    db.delete_user(p.email.as_ref().unwrap()).await.unwrap();
    assert!(
        !db.username_exists(p.email.as_ref().unwrap()).await.unwrap(),
        "user is deleted so username shouldn't exsit"
    );

    // register with email = None
    let mut p2 = p.clone();
    p2.email = None;
    db.register(&p2).await.unwrap();
    assert!(
        db.username_exists(p2.username).await.unwrap(),
        "user is registered so username should exsit"
    );
    assert!(
        !db.email_exists(p.email.as_ref().unwrap()).await.unwrap(),
        "user registration with email is deleted; so email shouldn't exsit"
    );

    // testing update email
    let update_email = UpdateEmail {
        username: p.username,
        new_email: p.email.as_ref().unwrap(),
    };
    db.update_email(&update_email).await.unwrap();
    println!(
        "null user email: {}",
        db.email_exists(p.email.as_ref().unwrap()).await.unwrap()
    );
    assert!(
        db.email_exists(p.email.as_ref().unwrap()).await.unwrap(),
        "user was with empty email but email is set; so email should exsit"
    );

    /*
     * test notification workflows
     * 1. Add notifications: a minimum of two, to mark as read and test if it has affected it
     * 2. Get unread notifications
     * 3. Mark a notification read, check if it has affected Step #2
     */

    // 1. add notification
    db.create_notification(an).await.unwrap();
    db.create_notification(an).await.unwrap();

    // 2. Get notifications
    let notifications = db.get_all_unread_notifications(&an.to).await.unwrap();
    assert_eq!(notifications.len(), 2);
    assert_eq!(notifications[0].heading.as_ref().unwrap(), an.heading);

    // 3. mark a notification read
    db.mark_notification_read(an.to, notifications[0].id.unwrap())
        .await
        .unwrap();
    let new_notifications = db.get_all_unread_notifications(&an.to).await.unwrap();
    assert_eq!(new_notifications.len(), 1);

    // create captcha
    db.create_captcha(p.username, c).await.unwrap();
    assert!(db.captcha_exists(None, c.key).await.unwrap());
    assert!(db.captcha_exists(Some(p.username), c.key).await.unwrap());

    // get captcha cooldown duration
    assert_eq!(db.get_captcha_cooldown(c.key).await.unwrap(), c.duration);

    // add traffic pattern
    db.add_traffic_pattern(p.username, c.key, tp).await.unwrap();
    assert_eq!(
        &db.get_traffic_pattern(p.username, c.key).await.unwrap(),
        tp
    );

    // delete traffic pattern
    db.delete_traffic_pattern(p.username, c.key).await.unwrap();
    assert!(
        matches!(
            db.get_traffic_pattern(p.username, c.key).await,
            Err(DBError::TrafficPatternNotFound)
        ),
        "deletion successful; traffic pattern no longer exists"
    );

    // add captcha levels
    db.add_captcha_levels(p.username, c.key, l).await.unwrap();

    // get captcha levels with username
    let levels = db
        .get_captcha_levels(Some(p.username), c.key)
        .await
        .unwrap();
    assert_eq!(levels, l);
    // get captcha levels without username
    let levels = db.get_captcha_levels(None, c.key).await.unwrap();
    assert_eq!(levels, l);

    // delete captcha levels
    db.delete_captcha_levels(p.username, c.key).await.unwrap();

    // update captcha; set description = username and duration *= duration;
    let mut c2 = c.clone();
    c2.duration *= c2.duration;
    c2.description = p.username;
    db.update_captcha_metadata(p.username, &c2).await.unwrap();

    // update captcha key; set key = username;
    db.update_captcha_key(p.username, c.key, p.username)
        .await
        .unwrap();
    // checking for captcha with old key; shouldn't exist
    assert!(!db.captcha_exists(Some(p.username), c.key).await.unwrap());
    // checking for captcha with new key; shouldn exist
    assert!(db
        .captcha_exists(Some(p.username), p.username)
        .await
        .unwrap());

    // delete captcha; updated key = p.username so invoke delete with it
    db.delete_captcha(p.username, p.username).await.unwrap();
    assert!(!db.captcha_exists(Some(p.username), c.key).await.unwrap());
}
