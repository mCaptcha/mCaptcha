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

use super::account::routes::Account;
use super::auth::routes::Auth;
use super::mcaptcha::captcha::routes::MCaptcha;
use super::mcaptcha::duration::routes::Duration;
use super::mcaptcha::levels::routes::Levels;
use super::meta::routes::Meta;
use super::notifications::routes::Notifications;
use super::pow::routes::PoW;

pub const ROUTES: Routes = Routes::new();

pub struct Routes {
    pub auth: Auth,
    pub account: Account,
    pub levels: Levels,
    pub mcaptcha: MCaptcha,
    pub duration: Duration,
    pub meta: Meta,
    pub pow: PoW,
    pub notifications: Notifications,
}

impl Routes {
    const fn new() -> Routes {
        Routes {
            auth: Auth::new(),
            account: Account::new(),
            levels: Levels::new(),
            mcaptcha: MCaptcha::new(),
            duration: Duration::new(),
            meta: Meta::new(),
            pow: PoW::new(),
            notifications: Notifications::new(),
        }
    }
}
