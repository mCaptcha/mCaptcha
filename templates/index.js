import {Router} from './router';

import * as login from './auth/login';
import * as register from './auth/register';
import * as panel from './panel/index';
import './auth/forms.scss';
import './panel/main.scss';
import VIEWS from './views/v1/routes';


const router = new Router();

router.register(VIEWS.panelHome, panel.index);
router.register(VIEWS.registerUser, register.index);
router.register(VIEWS.loginUser, login.index);
//router.register('/panel/layout.html/', panel.index);

router.route();
