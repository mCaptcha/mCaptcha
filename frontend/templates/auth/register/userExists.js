import ROUTES from '../../api/v1/routes';

import genJsonPayload from '../../utils/genJsonPayload';


//export const checkUsernameExists = async () => {
async function userExists()  {
  let username = document.getElementById('username');
  let val = username.value;
  let payload = {
    val,
  };

  //  return fetch(ROUTES.usernameExists, genJsonPayload(payload)).then(res => {
  //    if (res.ok) {
  //      res.json().then(data => {
  //        if (data.exists) {
  //          username.className += ' form__in-field--warn';
  //          alert('Username taken');
  //        }
  //        return data.exists;
  //      });
  //    } else {
  //      res.json().then(err => alert(`error: ${err.error}`));
  //    }
  //  });
  //

  let res = await fetch(ROUTES.usernameExists, genJsonPayload(payload));
  if (res.ok) {
    let data = await res.json();
    if (data.exists) {
      username.className += ' form__in-field--warn';
      alert('Username taken');
    }
    return data.exists;
  } else {
    let err = await res.json();
    alert(`error: ${err.error}`);
  }
  return false;
};

export default userExists;
