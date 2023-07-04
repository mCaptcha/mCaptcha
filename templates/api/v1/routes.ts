// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
