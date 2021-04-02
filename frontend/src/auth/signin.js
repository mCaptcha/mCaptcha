import isBlankString from '../utils/isBlankString';
import genJsonPayload from '../utils/genJsonPayload';

import ROUTES from '../api/v1/routes';

const signin = e => {
  e.preventDefault();
  let username = document.getElementById('username').value;
  isBlankString(e, username, 'username');

  let password = document.getElementById('password').value;
  let payload = {
    username,
    password,
  };

  fetch(ROUTES.loginUser, genJsonPayload(payload)).then(res => {
    if (res.ok) {
      alert('success');
    } else {
      res.json().then(err => alert(`error: ${err.error}`));
    }
  });
};

export default signin;
