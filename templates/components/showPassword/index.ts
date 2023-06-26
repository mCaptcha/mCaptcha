// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
