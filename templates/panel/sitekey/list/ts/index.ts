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

export const index = () => {
  registerCopySitekey();
};

const SITEKEY_COPY_ICON = `sitekey__copy-icon`;
const SITEKEY_COPY_DONE_ICON = `sitekey__copy-done-icon`;

const registerCopySitekey = () => {
  const icons = document.querySelectorAll(`.${SITEKEY_COPY_ICON}`);
  icons.forEach(icon => {
    icon.addEventListener('click', e => copySitekey(e));
  });
};

/*
 * Copy sitekey to clipboard
 */
const copySitekey = async (e: Event) => {
  const image = <HTMLElement>e.target;
  if (!image.classList.contains(SITEKEY_COPY_ICON)) {
    throw new Error(
      'This method should only be called when sitekey copy button/icon is clicked',
    );
  }
  const copyDoneIcon = <HTMLElement>(
    image.parentElement.querySelector(`.${SITEKEY_COPY_DONE_ICON}`)
  );
  const sitekey = image.dataset.sitekey;
  await navigator.clipboard.writeText(sitekey);
  image.style.display = 'none';
  copyDoneIcon.style.display = 'block';
  setTimeout(() => {
    copyDoneIcon.style.display = 'none';
    image.style.display = 'block';
  }, 1200);
};
