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

const normalizeUri = uri => {
  if (!uri) {
    throw new Error('uri is empty');
  }

  if (typeof uri !== 'string') {
    throw new TypeError('URI must be a string');
  }

  let uriLength = uri.length;
  if (uri[uriLength - 1] == '/') {
    uri = uri.slice(0, uriLength - 1);
  }
  return uri;
};

export class Router {
  constructor() {
    this.routes = [];
  }

  register(uri, fn) {
    // typechecks
    if (!uri) {
      throw new Error('uri is empty');
    }

    if (!fn) {
      throw new Error('fn is empty');
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

    const route = {
      uri,
      fn,
    };
    this.routes.push(route);
  }

  route() {
    this.routes.forEach(route => {
      // normalize for trailing slash
      let pattern = new RegExp(`^${route.uri}$`);
      let path = window.location.pathname;
      path = normalizeUri(path);
      if (path.match(pattern)) {
        return route.fn.call();
      }
    });
  }
}
