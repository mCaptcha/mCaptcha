import ROUTES from '../../api/v1/routes';

import isBlankString from '../../utils/genJsonPayload';
import genJsonPayload from '../../utils/genJsonPayload';

const login = e => {
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

export default login;
