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

import * as login from './auth/login';
import * as register from './auth/register';
import * as panel from './panel/index';
import * as addSiteKey from './panel/add-site-key/';

import VIEWS from './views/v1/routes';

import './auth/forms.scss';
import './panel/main.scss';
import './panel/header/sidebar/main.scss';
import './panel/taskbar/main.scss';
import './panel/help-banner/main.scss';
import './panel/add-site-key/main.scss';
import './panel/site-keys/main.scss';
import './errors/main.scss';

const router = new Router();

router.register(VIEWS.panelHome, panel.index);
router.register(VIEWS.registerUser, register.index);
router.register(VIEWS.loginUser, login.index);
router.register(VIEWS.addSiteKey, addSiteKey.index);

router.route();
