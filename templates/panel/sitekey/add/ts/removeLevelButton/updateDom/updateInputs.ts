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

const updateInput = (levelGroup: Element, newLevel: number) => {
  const inputs = <NodeListOf<HTMLInputElement>>(
    levelGroup.querySelectorAll(`.${CONST.LEVEL_INPUT_CLASS}`)
  );
  log.log(inputs);
  inputs.forEach(input => {
    if (input.id.includes(CONST.VISITOR_WITHOUT_LEVEL)) {
      log.log(`${input.id}`);
      log.log('changing visitor_threshold input');
      const id = `${CONST.VISITOR_WITHOUT_LEVEL}${newLevel}`;
      input.id = id;
      input.name = id;
    }

    if (input.id.includes(CONST.DIFFICULTY_WITHOUT_LEVEL)) {
      log.log('changing difficulty input');
      const id = `${CONST.DIFFICULTY_WITHOUT_LEVEL}${newLevel}`;
      input.id = id;
      input.name = id;
    }
  });
};

export default updateInput;
