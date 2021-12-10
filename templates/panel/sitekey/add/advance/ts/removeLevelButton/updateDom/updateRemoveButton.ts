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
import CONST from "../../const";

/** update remove level button's ID */
const updateRemoveButton = (levelGroup: Element, newLevel: number): void => {
  // rename button
  const button = <HTMLInputElement>(
    levelGroup.querySelector(`.${CONST.REMOVE_LEVEL_BUTTON_CLASS}`)
  );
  const id = `${CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL}${newLevel}`;
  button.id = id;
  button.name = id;
};

export default updateRemoveButton;
