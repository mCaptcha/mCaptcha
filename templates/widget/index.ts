/*
 * Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
import "./main.scss";
//import prove from './runner/prove';
//import fetchPoWConfig from './runner/fetchPoWConfig';
//import sendWork from './runner/sendWork';
//import sendToParent from './runner/sendToParent';
//import * as CONST from './runner/const';
//
///** add  mcaptcha widget element to DOM */
//export const register = () => {
//  const verificationContainer = <HTMLElement>(
//    document.querySelector('.widget__verification-container')
//  );
//  verificationContainer.style.display = 'flex';
//
//  CONST.btn().addEventListener('click', e => solveCaptchaRunner(e));
//};
//
//const solveCaptchaRunner = async (e: Event) => {
//  e.preventDefault();
//  // steps:
//
//  // 1. hide --before message
//  CONST.messageText().before().style.display = 'none';
//
//  // 1. show --during
//  CONST.messageText().during().style.display = 'block';
//  // 1. get config
//  const config = await fetchPoWConfig();
//  // 2. prove work
//  const proof = await prove(config);
//  // 3. submit work
//  const token = await sendWork(proof);
//  // 4. send token
//  sendToParent(token);
//  // 5. mark checkbox checked
//  CONST.btn().checked = true;
//};
//
//register();
