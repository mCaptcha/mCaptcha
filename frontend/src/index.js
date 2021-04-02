import './css/forms.scss';

import signin from './auth/signin';
import registerUser from './auth/register';
import {checkUsernameEventHandler} from './auth/userExists';

if (window.location.pathname == '/') {
  let form = document.getElementById('form');
  form.addEventListener('submit', signin, true);
} else if (window.location.pathname == '/signup') {
  let form = document.getElementById('form');
  form.addEventListener('submit', registerUser, true);
  let username = document.getElementById('username');
  username.addEventListener('input', checkUsernameEventHandler, false);
} else {
}


//export default signin;
