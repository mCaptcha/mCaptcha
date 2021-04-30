import {Router} from './router';

import * as login from './auth/login';
import * as register from './auth/register';
import * as panel from './panel/index';
import './auth/forms.scss';
import './panel/main.scss';

const router = new Router();

router.register('/', panel.index);
router.register('/register', register.index);
router.register('/login', login.index);
router.register('/panel/layout.html/', panel.index);

router.route();
