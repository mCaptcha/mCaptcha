// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
            "user is deleted so username shouldn't exist"
        );
    }

    assert!(matches!(
        db.get_secret(&p.username).await,
        Err(DBError::AccountNotFound)
    ));

    db.register(p).await.unwrap();

    assert!(matches!(db.register(&p).await, Err(DBError::UsernameTaken)));

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

    // testing get_email
    assert_eq!(
        db.get_email(p.username)
            .await
            .unwrap()
            .as_ref()
            .unwrap()
            .as_str(),
        p.email.unwrap()
    );

    // testing email exists
    assert!(
        db.email_exists(p.email.as_ref().unwrap()).await.unwrap(),
        "user is registered so email should exist"
    );
    assert!(
        db.username_exists(p.username).await.unwrap(),
        "user is registered so username should exist"
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
        "user is deleted so username shouldn't exist"
    );

    // register with email = None
    let mut p2 = p.clone();
    p2.email = None;
    db.register(&p2).await.unwrap();
    assert!(
        db.username_exists(p2.username).await.unwrap(),
        "user is registered so username should exist"
    );
    assert!(
        !db.email_exists(p.email.as_ref().unwrap()).await.unwrap(),
        "user registration with email is deleted; so email shouldn't exist"
    );

    // testing get_email = None
    assert_eq!(db.get_email(p.username).await.unwrap(), None);

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
        "user was with empty email but email is set; so email should exist"
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
    let notifications = db.get_all_unread_notifications(an.to).await.unwrap();
    assert_eq!(notifications.len(), 2);
    assert_eq!(notifications[0].heading.as_ref().unwrap(), an.heading);

    // 3. mark a notification read
    db.mark_notification_read(an.to, notifications[0].id.unwrap())
        .await
        .unwrap();
    let new_notifications = db.get_all_unread_notifications(an.to).await.unwrap();
    assert_eq!(new_notifications.len(), 1);

    // create captcha
    db.create_captcha(p.username, c).await.unwrap();
    assert!(db.captcha_exists(None, c.key).await.unwrap());
    assert!(db.captcha_exists(Some(p.username), c.key).await.unwrap());

    // get secret from captcha key
    let secret_from_captcha = db.get_secret_from_captcha(&c.key).await.unwrap();
    assert_eq!(secret_from_captcha.secret, p.secret, "user secret matches");

    // get captcha configuration
    let captcha = db.get_captcha_config(p.username, c.key).await.unwrap();
    assert_eq!(captcha.key, c.key);
    assert_eq!(captcha.duration, c.duration);
    assert_eq!(captcha.description, c.description);

    // get all captchas that belong to user
    let all_user_captchas = db.get_all_user_captchas(p.username).await.unwrap();
    assert_eq!(all_user_captchas.len(), 1);
    assert_eq!(all_user_captchas[0], captcha);

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

    /*
     * Test stats
     * 1. record fetch config
     * 2. record solve
     * 3. record token verify
     * 4. fetch config fetches
     * 5. fetch solves
     * 6. fetch token verify
     */

    assert!(db
        .fetch_config_fetched(p.username, c.key)
        .await
        .unwrap()
        .is_empty());
    assert!(db.fetch_solve(p.username, c.key).await.unwrap().is_empty());
    assert!(db
        .fetch_confirm(p.username, c.key)
        .await
        .unwrap()
        .is_empty());

    db.record_fetch(c.key).await.unwrap();
    db.record_solve(c.key).await.unwrap();
    db.record_confirm(c.key).await.unwrap();

    // analytics start

    db.analytics_create_psuedo_id_if_not_exists(c.key)
        .await
        .unwrap();
    let psuedo_id = db
        .analytics_get_psuedo_id_from_capmaign_id(c.key)
        .await
        .unwrap();
    db.analytics_create_psuedo_id_if_not_exists(c.key)
        .await
        .unwrap();
    assert_eq!(
        psuedo_id,
        db.analytics_get_psuedo_id_from_capmaign_id(c.key)
            .await
            .unwrap()
    );
    assert_eq!(
        c.key,
        db.analytics_get_capmaign_id_from_psuedo_id(&psuedo_id)
            .await
            .unwrap()
    );

    let analytics = CreatePerformanceAnalytics {
        time: 0,
        difficulty_factor: 0,
        worker_type: "wasm".into(),
    };
    db.analysis_save(c.key, &analytics).await.unwrap();
    let limit = 50;
    let mut offset = 0;
    let a = db.analytics_fetch(c.key, limit, offset).await.unwrap();
    assert_eq!(a[0].time, analytics.time);
    assert_eq!(a[0].difficulty_factor, analytics.difficulty_factor);
    assert_eq!(a[0].worker_type, analytics.worker_type);
    offset += 1;
    assert!(db
        .analytics_fetch(c.key, limit, offset)
        .await
        .unwrap()
        .is_empty());

    db.analytics_delete_all_records_for_campaign(c.key)
        .await
        .unwrap();
    assert_eq!(db.analytics_fetch(c.key, 1000, 0).await.unwrap().len(), 0);
    assert!(!db.analytics_captcha_is_published(c.key).await.unwrap());
    db.analytics_delete_all_records_for_campaign(c.key)
        .await
        .unwrap();
    // analytics end

    assert_eq!(db.fetch_solve(p.username, c.key).await.unwrap().len(), 1);
    assert_eq!(
        db.fetch_config_fetched(p.username, c.key)
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(db.fetch_solve(p.username, c.key).await.unwrap().len(), 1);
    assert_eq!(db.fetch_confirm(p.username, c.key).await.unwrap().len(), 1);

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

    // delete captcha levels
    db.delete_captcha_levels(p.username, c.key).await.unwrap();

    // update captcha; set description = username and duration *= duration;
    let mut c2 = c.clone();
    c2.duration *= c2.duration;
    c2.description = p.username;
    db.update_captcha_metadata(p.username, &c2).await.unwrap();

    // delete captcha; updated key = p.username so invoke delete with it
    db.delete_captcha(p.username, p.username).await.unwrap();
    assert!(!db.captcha_exists(Some(p.username), c.key).await.unwrap());
}
