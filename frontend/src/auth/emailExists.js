import ROUTES from '../api/v1/routes';

import genJsonPayload from '../utils/genJsonPayload';

const checkEmailExists = async () => {
  let email = document.getElementById('email');
  let val = email.value;
  let payload = {
    val,
  };

  //  return fetch(ROUTES.emailExists, genJsonPayload(payload)).then(res => {
  //    if (res.ok) {
  //      res.json().then(data => {
  //        if (data.exists) {
  //          console.log(email.className);
  //          email.className += ' form__in-field--warn';
  //          alert('Email taken');
  //        }
  //
  //        return data.exists;
  //      });
  //    } else {
  //      res.json().then(err => alert(`error: ${err.error}`));
  //    }
  //  });
  //

  let res = await fetch(ROUTES.emailExists, genJsonPayload(payload));
  if (res.ok) {
    let data = await res.json();
    if (data.exists) {
      email.className += ' form__in-field--warn';
      alert('Email taken');
    }
    return data.exists;
  } else {
    let err = await res.json();
    alert(`error: ${err.error}`);
  }
};

export {checkEmailExists};
