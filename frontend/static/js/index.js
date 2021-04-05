import './css/forms.scss';

import signin from './auth/signin';
import registerUser from './auth/register';
import {run as runPanel} from './panel/index';
import {checkUsernameEventHandler} from './auth/userExists';

if (window.location.pathname == '/') {
  let form = document.getElementById('form');
  form.addEventListener('submit', signin, true);
} else if (window.location.pathname == '/signup') {
  let form = document.getElementById('form');
  form.addEventListener('submit', registerUser, true);
  let username = document.getElementById('username');
  username.addEventListener('input', checkUsernameEventHandler, false);
} else if (window.location.pathname.includes('panel')) {
  runPanel();
} else {
}

//export default signin;
