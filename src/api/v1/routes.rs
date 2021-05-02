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

use lazy_static::lazy_static;

use super::account::routes::Account;
use super::auth::routes::Auth;
use super::mcaptcha::duration::routes::Duration;
use super::mcaptcha::levels::routes::Levels;
use super::mcaptcha::mcaptcha::routes::MCaptcha;

lazy_static! {
    pub static ref ROUTES: Routes = Routes::default();
}

#[derive(Default)]
pub struct Routes {
    pub auth: Auth,
    pub account: Account,
    pub levels: Levels,
    pub mcaptcha: MCaptcha,
    pub duration: Duration,
}
