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

import {Router} from './router';

import * as login from './auth/login/ts/';
import * as register from './auth/register/ts/';
import * as panel from './panel/ts/index';
import settings from './panel/settings/';
import * as deleteAccount from './panel/settings/account/delete';
import * as updateSecret from './panel/settings/secret/update';
import * as addSiteKey from './panel/sitekey/add/ts';
import * as editSitekey from './panel/sitekey/edit/';
import * as deleteSitekey from './panel/sitekey/delete/';
import * as listSitekeys from './panel/sitekey/list/ts';
import * as notidications from './panel/notifications/ts';
import {MODE} from './logger';
import log from './logger';

import VIEWS from './views/v1/routes';

import './main.scss';
import './auth/css/main.scss';
import './components/details-footer/main.scss';
import './components/error/main.scss';
import './components/showPassword/main.scss';
import './panel/css/main.scss';
import './panel/navbar/main.scss';
import './panel/settings/main.scss';
import './panel/notifications/main.scss';
import './panel/header/taskbar/main.scss';
import './panel/help-banner/main.scss';
import './panel/sitekey/add/css/main.scss';
import './panel/sitekey/list/css/main.scss';

import './errors/main.scss';

log.setMode(MODE.production);

const router = new Router();

router.register(VIEWS.panelHome, panel.index);
router.register(VIEWS.settings, settings);
router.register(VIEWS.deleteAccount, deleteAccount.index);
router.register(VIEWS.updateSecret, updateSecret.index);
router.register(VIEWS.registerUser, register.index);
router.register(VIEWS.loginUser, login.index);
router.register(VIEWS.notifications, notidications.index);
router.register(VIEWS.listSitekey, listSitekeys.index);
router.register(VIEWS.addSiteKey, addSiteKey.index);
router.register(VIEWS.editSitekey('[A-Z),a-z,0-9]+'), editSitekey.index);
router.register(VIEWS.deleteSitekey('[A-Z),a-z,0-9]+'), deleteSitekey.index);

try {
  router.route();
} catch (e) {
  console.log(e);
}
