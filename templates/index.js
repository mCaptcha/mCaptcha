import {Router} from './router';

import * as login from './auth/login';
import * as register from './auth/register';
import * as panel from './panel/index';

const router = new Router();

router.register('/', login.index);
router.register('/register', register.index);
router.register('/panel/', panel.index);
router.register('/panel/layout.html/', panel.index);

router.route();
