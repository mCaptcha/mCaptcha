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

/** 
 * querySelector is the the selector that will be 
 * used to fetch elements.
 * So when using class-names, pass in ".whatever-classname"
 * and for ID, "#id".
 * */
const getFormUrl = (querySelector: null|string|HTMLFormElement) => {
  let form;
  if (querySelector === null) {
    form = <HTMLFormElement>document.querySelector("form");
  } 
  if (querySelector === "string" || querySelector instanceof String) {
    form = <HTMLFormElement>document.querySelector(querySelector.toString());
  } 
  if (querySelector instanceof HTMLFormElement) {
      form = querySelector;
  }

  if ( form !== undefined) {
  return form.action
  } else {
    throw new Error("Can't find form");
  }
};

export default getFormUrl;
