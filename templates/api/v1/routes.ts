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

const ROUTES = {
  registerUser: "/api/v1/signup",
  loginUser: "/api/v1/signin",
  signoutUser: "/api/v1/signout",
  deleteAccount: "/api/v1/account/delete",
  usernameExists: "/api/v1/account/username/exists",
  emailExists: "/api/v1/account/email/exists",
  healthCheck: "/api/v1/meta/health",
  buildDetails: "/api/v1/meta/build",
  markNotificationRead: "/api/v1/notifications/read",
};

export default ROUTES;
