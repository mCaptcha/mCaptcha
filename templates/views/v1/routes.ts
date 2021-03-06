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
  registerUser: '/join/',
  loginUser: '/login/',
  signoutUser: '/api/v1/signout',
  panelHome: '/',
  settings: '/settings/',
  updateSecret: '/settings/secret/update/',
  deleteAccount: '/settings/account/delete/',
  docsHome: '/docs/',
  notifications: '/notifications',
  listSitekey: '/sitekeys/',
  viewSitekey: (key: string) => `/sitekey/${key}/`,
  editSitekey: (key: string) => `/sitekey/${key}/edit/`,
  deleteSitekey: (key: string) => `/sitekey/${key}/delete/`,
  addSiteKey: '/sitekeys/add',
};

export default ROUTES;
