import * as login from './auth/login';
import * as register from './auth/register';
import * as panel from './panel/index';

if (window.location.pathname == '/') {
  login.index();
} else if (window.location.pathname == '/register') {
  register.index();
//  let form = document.getElementById('form');
//  form.addEventListener('submit', registerUser, true);
//  let username = document.getElementById('username');
//  username.addEventListener('input', checkUsernameEventHandler, false);
} else if (window.location.pathname.includes('panel')) {
  panel.index();
} else {
}

//export default signin;
