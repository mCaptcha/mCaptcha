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
