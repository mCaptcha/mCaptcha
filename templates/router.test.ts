// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later
"use strict";

import {Router} from "./router";


const result = {
  result: "",
};

const panelResult = "hello from panel";
const panelRoute = "/panel";
const panel = () => (result.result = panelResult);

const settingsRoute = "/sitekey/";
const settingsResult = "hello from settings";
const settings = () => (result.result = settingsResult);

const patternRoute = "/sitekey/[A-Z,a-z,0-9,_]+/";
const examplePatternRoute = "/sitekey/alksdjakdjadajkhdjahrjke234/";
const patterResult = "hello from pattern route";
const pattern = () => (result.result = patterResult);

const UriExistsErr = "URI exists";
const emptyUriErr = "uri is empty";
const unregisteredRouteErr = "Route isn't registered";

const router = new Router();
router.register(patternRoute, pattern);
router.register(panelRoute, panel);
router.register(settingsRoute, settings);

it("checks if Router works", () => {
  window.history.pushState({}, "", examplePatternRoute);
  router.route();
  expect(result.result).toBe(patterResult);

  window.history.pushState(
    {},
    "",
    examplePatternRoute.slice(0, examplePatternRoute.length - 1),
  );
  router.route();
  expect(result.result).toBe(patterResult);

  window.history.pushState({}, "Settings", settingsRoute);
  router.route();
  expect(result.result).toBe(settingsResult);
  window.history.pushState({}, "Panel", panelRoute);
  router.route();
  expect(result.result).toBe(panelResult);

  // duplicate URI registration
  try {
    router.register(settingsRoute, settings);
  } catch (e) {
    expect(e.message).toBe(UriExistsErr);
  }

  // empty URI registration
  try {
    router.register("      ", settings);
  } catch (e) {
    expect(e.message).toBe(emptyUriErr);
  }

  // routing to unregistered route
  try {
    window.history.pushState({}, "Page Doesn't Exist", "/page/doesnt/exist");
    router.route();
  } catch (e) {
    expect(e.message).toBe(unregisteredRouteErr);
  }

  // routing to unregistered route
  try {
    window.history.pushState({}, "Page Doesn't Exist", "/sitekey/;asd;lasdj");
    router.route();
  } catch (e) {
    expect(e.message).toBe(unregisteredRouteErr);
  }
});
