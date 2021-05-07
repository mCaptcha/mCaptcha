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
import CONST from '../../const';
import log from '../../../../../../logger';

/** update level lables to match new level */
const updateLabels = (levelGroup: Element, newLevel: number) => {
  // rename labels
  const labels = <NodeListOf<HTMLLabelElement>>(
    levelGroup.querySelectorAll(`label`)
  );
  log.log(labels);
  labels.forEach(label => {
    log.log(`${label.htmlFor}`);
    const currentFor = label.htmlFor;
    if (currentFor.includes(CONST.VISITOR_WITHOUT_LEVEL)) {
      label.htmlFor = `${CONST.VISITOR_WITHOUT_LEVEL}${newLevel}`;
    } else if (currentFor.includes(CONST.DIFFICULTY_WITHOUT_LEVEL)) {
      label.htmlFor = `${CONST.DIFFICULTY_WITHOUT_LEVEL}${newLevel}`;
    } else if (
      currentFor.includes(CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL)
    ) {
      label.htmlFor = `${CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL}${newLevel}`;
    } else {
      if (currentFor != 'add') {
        throw new Error(
          `Did you add an extra label to DOM? Found label with for: ${currentFor}`,
        );
      }
    }
  });
};

export default updateLabels;
