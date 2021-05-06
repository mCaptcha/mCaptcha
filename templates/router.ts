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

/** Removes trailing slashed from URI */
const normalizeUri = (uri: string) => {
  if (typeof uri == 'string') {
    if (uri.trim().length == 0) {
      throw new Error('uri is empty');
    }

    let uriLength = uri.length;
    if (uri[uriLength - 1] == '/') {
      uri = uri.slice(0, uriLength - 1);
    }
    return uri;
  } else {
    throw new TypeError(`${typeof uri} ${uri}`);
  }
};

/** URI<-> Fn mapping type */
type routeTuple = {
  uri: string;
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
  register(uri: string, fn: () => void) {
    // typechecks
    if (uri.trim().length == 0) {
      throw new Error('uri is empty');
    }

    if (typeof uri !== 'string') {
      throw new TypeError('URI must be a string');
    }

    if (typeof fn !== 'function') {
      throw new TypeError('a callback fn must be provided');
    }

    this.routes.forEach(route => {
      if (route.uri == uri) {
        throw new Error(
          `URI exists. provided URI: ${uri}, registered config: ${route}`,
        );
      }
    });

    uri = normalizeUri(uri);

    const route: routeTuple = {
      uri,
      fn,
    };
    this.routes.push(route);
  }

  /**
   * executes registered function with route
   * matches window.pathname.location
   * */
  route() {
    const path = normalizeUri(window.location.pathname);

    this.routes.forEach(route => {
      const pattern = new RegExp(`^${route.uri}$`);
      if (path.match(pattern)) {
        return route.fn();
      }
    });
  }
}
