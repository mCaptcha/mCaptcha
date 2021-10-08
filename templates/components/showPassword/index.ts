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

const showPasswordButtonClassHidden = "show-password--hide";
const showPasswordButtonClassShowing = "show-password--show";

const container = "show-password-container";

let display = "hidden";

const showPasswordButtons = () => {
  let buttons: NodeListOf<HTMLElement>;

  return (() => {
    if (buttons === undefined) {
      buttons = <NodeListOf<HTMLElement>>(
        document.querySelectorAll(`.${showPasswordButtonClassShowing}`)
      );
    }
    return buttons;
  })();
};

const hidePasswordButtons = () => {
  let buttons: NodeListOf<HTMLElement>;

  return (() => {
    if (buttons === undefined) {
      buttons = <NodeListOf<HTMLElement>>(
        document.querySelectorAll(`.${showPasswordButtonClassHidden}`)
      );
    }
    return buttons;
  })();
};

// e is click event from show password container
export const showPassword = (): void => {
  const inputs = document.body.querySelectorAll("input");

  if (display == "hidden") {
    display = "show";
    inputs.forEach(element => {
      if (element.type === "password") {
        element.type = "text";
      }
    });
    showPasswordButtons().forEach((button: HTMLInputElement) => {
      button.style.display = "none";
    });

    hidePasswordButtons().forEach((button: HTMLInputElement) => {
      button.style.display = "inline";
    });
  } else {
    display = "hidden";
    inputs.forEach(element => {
      if (element.type === "text" && element.name.includes("password")) {
        element.type = "password";
      }
    });
    showPasswordButtons().forEach((button: HTMLInputElement) => {
      button.style.display = "inline";
    });

    hidePasswordButtons().forEach((button: HTMLInputElement) => {
      button.style.display = "none";
    });
  }

  // posibily clicked on something else
};

export const registerShowPassword = (): void => {
  document.querySelectorAll(`.${container}`).forEach(container => {
    container.addEventListener("click", showPassword);
  });
};

export default registerShowPassword;
