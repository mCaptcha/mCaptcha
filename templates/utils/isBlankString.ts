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
import createError from "../components/error/";

const isBlankString = (value: string|number, field: string, event?: Event): boolean => {
  value = value.toString();
  if (!value.replace(/\s/g, "").length) {
    if (event !== undefined) {
      event.preventDefault();
    }
    const msg = `${field} can't be empty`;
    createError(msg);
    throw new  Error(msg);
  }
  return false;
};

export default isBlankString;
