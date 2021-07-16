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

const ROUTES = {
  registerUser: '/api/v1/signup',
  loginUser: '/api/v1/signin',
  signoutUser: '/api/v1/signout',
  deleteAccount: '/api/v1/account/delete',
  usernameExists: '/api/v1/account/username/exists',
  emailExists: '/api/v1/account/email/exists',
  healthCheck: '/api/v1/meta/health',
  buildDetails: '/api/v1/meta/build',
  addDomain: '/api/v1/mcaptcha/domain/add',
  challengeDomain: '/api/v1/mcaptcha/domain/domain/verify/challenge/get',
  proveDomain: '/api/v1/mcaptcha/domain/domain/verify/challenge/prove',
  deleteDomain: '/api/v1/mcaptcha/domain/delete',
  addToken: '/api/v1/mcaptcha/domain/token/add',
  updateTokenKey: '/api/v1/mcaptcha/domain/token/update',
  getTokenKey: '/api/v1/mcaptcha/domain/token/get',
  deleteToken: '/api/v1/mcaptcha/domain/token/delete',
  addTokenLevels: '/api/v1/mcaptcha/domain/token/levels/add',
  updateTokenLevels: '/api/v1/mcaptcha/domain/token/levels/update',
  deleteTokenLevels: '/api/v1/mcaptcha/domain/token/levels/delete',
  getTokenLevels: '/api/v1/mcaptcha/domain/token/levels/get',
  getTokenDuration: '/api/v1/mcaptcha/domain/token/token/get',
  updateTokenDuration: '/api/v1/mcaptcha/domain/token/token/update',
  markNotificationRead: '/api/v1/notifications/read',
};

export default ROUTES;
