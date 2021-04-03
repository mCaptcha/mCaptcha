import './css/forms.scss';
import registerUser from './auth/register';

let form = document.getElementById('form');
form.addEventListener('submit', registerUser, true);

let username = document.getElementById('username');
username.addEventListener('input', checkUsernameEventHandler, false);
