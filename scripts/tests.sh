#!/bin/bash
# hotfix for DB error: too many connections, can't create new client
#
# I tried running cargo test with the `--jobs` parameter set to 1 but that didn't 
# seem to solve the issue. This scr will run the whole test suite but one test at a time. 


for ut in  \
	api::v1::meta::tests::build_details_works \
	api::v1::mcaptcha::easy::tests::isoloated_test::easy_configuration_works \
	api::v1::meta::tests::health_works \
	api::v1::pow::tests::scope_pow_works \
	api::v1::account::test::uname_email_exists_works \
	api::v1::mcaptcha::easy::tests::easy_works \
	api::v1::pow::get_config::tests::get_pow_config_works \
	api::v1::pow::verify_pow::tests::verify_pow_works \
	api::v1::mcaptcha::update::tests::update_and_get_mcaptcha_works \
	date::tests::print_date_test \
	api::v1::tests::auth::serverside_password_validation_works \
	docs::tests::docs_works \
	email::verification::tests::email_verification_works \
	errors::tests::error_works \
	pages::errors::tests::error_pages_work \
	pages::panel::notifications::tests::print_date_test \
	api::v1::notifications::add::tests::notification_works \
	api::v1::account::test::username_update_works \
	pages::panel::sitekey::tests::get_sitekey_routes_work \
	api::v1::mcaptcha::test::level_routes_work \
	pages::routes::tests::sitemap_works \
	api::v1::tests::protected::protected_routes_work \
	pages::tests::public_pages_tempaltes_work \
	static_assets::filemap::tests::filemap_works \
	static_assets::static_files::tests::favicons_work \
	static_assets::static_files::tests::static_assets_work \
	pages::tests::protected_pages_templates_work \
	test::version_source_code_url_works \
	widget::test::captcha_widget_route_works \
	pages::panel::sitekey::edit::test::edit_sitekey_work \
	api::v1::pow::verify_token::tests::validate_captcha_token_works \
	api::v1::notifications::get::tests::notification_get_works \
	api::v1::notifications::mark_read::tests::notification_mark_read_works \
	api::v1::account::test::email_udpate_password_validation_del_userworks \
	api::v1::tests::auth::auth_works \
	pages::panel::sitekey::view::test::view_sitekey_work \
	api::v1::account::password::tests::update_password_works \
	pages::panel::sitekey::list::test::list_sitekeys_work
do
	cargo test -- $ut
done
