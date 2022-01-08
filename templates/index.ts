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

import { Router } from "./router";

import * as login from "./auth/login/ts/";
import * as register from "./auth/register/ts/";
import * as panel from "./panel/ts/index";
import settings from "./panel/settings/";
import * as deleteAccount from "./panel/settings/account/delete";
import * as updateSecret from "./panel/settings/secret/update";
import * as addSiteKeyAdvance from "./panel/sitekey/add/advance/ts";
import * as addSiteKeyEasy from "./panel/sitekey/add/novice/ts";
import * as editSitekeyAdvance from "./panel/sitekey/edit/";
import * as editSitekeyEasy from "./panel/sitekey/edit/easy/";
import * as deleteSitekey from "./panel/sitekey/delete/";
import * as listSitekeys from "./panel/sitekey/list/ts";
import * as notidications from "./panel/notifications/ts";
import { MODE } from "./logger";
import log from "./logger";

import VIEWS from "./views/v1/routes";

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
router.register(VIEWS.addSiteKeyAdvance,addSiteKeyAdvance.index);
router.register(VIEWS.addSiteKeyEasy, addSiteKeyEasy.index);
router.register(VIEWS.editSitekeyAdvance("[A-Z),a-z,0-9]+"), editSitekeyAdvance.index);
router.register(VIEWS.editSitekeyEasy("[A-Z),a-z,0-9]+"), editSitekeyEasy.index);
router.register(VIEWS.deleteSitekey("[A-Z),a-z,0-9]+"), deleteSitekey.index);

try {
  router.route();
} catch (e) {
  console.log(e);
}
