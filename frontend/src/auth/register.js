import ROUTES from '../api/v1/routes';

import isBlankString from '../utils/isBlankString';
import genJsonPayload from '../utils/genJsonPayload';

import {checkUsernameExists} from './userExists';
import {checkEmailExists} from './emailExists';

const registerUser = async e => {
  e.preventDefault();

  let username = document.getElementById('username').value;
  isBlankString(e, username, 'username');

  let password = document.getElementById('password').value;
  let passwordCheck = document.getElementById('password-check').value;
  if (password != passwordCheck) {
    return alert("passwords don't match, check again!");
  }

  let email = document.getElementById('email').value;
  isBlankString(e, email, 'email');

  let exists = await checkUsernameExists();
  if (exists) {
    return;
  }

  exists = await checkEmailExists();
  if (exists) {
    return;
  }

  let payload = {
    username,
    password,
    email,
  };

  let res = await fetch(ROUTES.registerUser, genJsonPayload(payload));
  if (res.ok) {
    alert('success');
  } else {
    let err = await res.json();
    alert(`error: ${err.error}`);
  }
};

export default registerUser;
