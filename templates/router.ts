// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

/** Removes trailing slash from URI */
const normalizeUri = (uri: string) => {
  uri = uri.trim();
  if (uri.length == 0) {
    throw new Error("uri is empty");
  }

  const uriLength = uri.length;
  if (uri[uriLength - 1] == "/") {
    uri = uri.slice(0, uriLength - 1);
  }
  return uri;
};

/** URI<-> Fn mapping type */
type routeTuple = {
  pattern: RegExp;
  fn: () => void;
};

/**
 * Router that selectively executes fucntions
 * based on window.location.pathname
 * */
export class Router {
  routes: Array<routeTuple>;
  constructor() {
    this.routes = [];
  }

  /**
   * registers a route-function pair with Router
   * @param {string} uri - route to be registered
   * @param {function} fn: - function to be registered when window.locatin.path
   * matches uri
   * */
  register(uri: string, fn: () => void): void {
    uri = normalizeUri(uri);

    const pattern = new RegExp(`^${uri}$`);

    const patterString = pattern.toString();
    if (
      this.routes.find((route) => {
        if (route.pattern.toString() == patterString) {
          return true;
        } else {
          return false;
        }
      })
    ) {
      throw new Error("URI exists");
    }

    const route: routeTuple = {
      pattern,
      fn,
    };
    this.routes.push(route);
  }

  /**
   * executes registered function with route
   * matches window.pathname.location
   * */
  route(): void {
    const path = normalizeUri(window.location.pathname);

    let fn: undefined | (() => void);

    if (
      this.routes.find((route) => {
        if (path.match(route.pattern)) {
          fn = route.fn;
          return true;
        }
      })
    ) {
      if (fn === undefined) {
        throw new Error("Route isn't registered");
      } else {
        return fn();
      }
    }
  }
}
