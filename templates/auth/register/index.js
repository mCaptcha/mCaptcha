import ROUTES from '../../api/v1/routes';
import VIEWS from '../../views/v1/routes';

import isBlankString from '../../utils/genJsonPayload';
import genJsonPayload from '../../utils/genJsonPayload';

import userExists from './userExists';
import {checkEmailExists} from './emailExists';

//import '../forms.scss';

const registerUser = async e => {
  e.preventDefault();

  let username = document.getElementById('username').value;
  isBlankString(e, username, 'username');

  let password = document.getElementById('password').value;
  let passwordCheck = document.getElementById('password-check').value;
  if (password != passwordCheck) {
    return alert("passwords don't match, check again!");
  }

  let exists = await userExists();
  if (exists) {
    return;
  }

  let email = document.getElementById('email').value;
  if (!email.replace(/\s/g, '').length) {
    email = null;
  } else {
    exists = await checkEmailExists();
    if (exists) {
      return;
    }
  }

  let payload = {
    username,
    password,
    email,
  };

  let res = await fetch(ROUTES.registerUser, genJsonPayload(payload));
  if (res.ok) {
    alert('success');
    window.location.assign(VIEWS.loginUser);
  } else {
    let err = await res.json();
    alert(`error: ${err.error}`);
  }
};

export const index = () => {
  let form = document.getElementById('form');
  form.addEventListener('submit', registerUser, true);

  let username = document.getElementById('username');
  username.addEventListener('input', userExists, false);
};
