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
import CopyIcon from "../../../../components/clipboard/";

const SITEKEY_COPY_ICON = "sitekey__copy-icon";
const SITEKEY_COPY_DONE_ICON = "sitekey__copy-done-icon";

export const index = (): void => {
  const image = document.querySelectorAll(`.${SITEKEY_COPY_ICON}`);
  image.forEach((img: HTMLElement) => {
    if (!img.classList.contains(SITEKEY_COPY_ICON)) {
      throw new Error(
        "This method should only be called when sitekey copy button/icon is clicked"
      );
    }
    const sitekey = img.dataset.sitekey;
    new CopyIcon(sitekey, img, SITEKEY_COPY_DONE_ICON);
  });
};
