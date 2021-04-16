import ROUTES from '../../api/v1/routes';
import VIEWS from '../../views/v1/routes';

import isBlankString from '../../utils/genJsonPayload';
import genJsonPayload from '../../utils/genJsonPayload';

//import '../forms.scss';

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
      window.location.assign(VIEWS.panelHome);
    } else {
      res.json().then(err => alert(`error: ${err.error}`));
    }
  });
};

export const index = () => {
  let form = document.getElementById('form');
  form.addEventListener('submit', login, true);
};
