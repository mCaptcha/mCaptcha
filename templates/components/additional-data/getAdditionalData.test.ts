/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
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

import additionalData from "./index";

it("sudo form works", () => {
  try {
    additionalData();
  } catch (e) {
    expect(e.message).toBe(
      "Couldn't retrieve additional data element, is the component loaded?",
    );
  }

  const element = document.createElement("div");
  element.id = "additional-data";
  document.body.appendChild(element);
  expect(additionalData()).toBe(element);
});
