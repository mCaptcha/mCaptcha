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
};

export default ROUTES;
