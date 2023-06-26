// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

const ROUTES = {
  registerUser: "/join/",
  loginUser: "/login/",
  signoutUser: "/api/v1/signout",
  panelHome: "/",
  settings: "/settings/",
  updateSecret: "/settings/secret/update/",
  deleteAccount: "/settings/account/delete/",
  docsHome: "/docs/",
  notifications: "/notifications",
  listSitekey: "/sitekeys/",
  viewSitekey: (key: string): string => `/sitekey/${key}/`,
  editSitekeyAdvance: (key: string): string => `/sitekey/${key}/advance/edit/`,
  addSiteKeyAdvance: "/sitekeys/advance/add",
  addSiteKeyEasy: "/sitekeys/easy/add",
  editSitekeyEasy: (key: string): string => `/sitekey/${key}/easy/edit/`,
  deleteSitekey: (key: string): string => `/sitekey/${key}/delete/`,
};

export default ROUTES;
