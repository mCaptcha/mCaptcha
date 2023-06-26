// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

const additionalData = (): HTMLElement => {
  let element = null;
  const ID = "additional-data";

  if (element === null) {
    element = <HTMLElement>document.getElementById(ID);
    if (element === undefined) {
      throw new Error(
        "Couldn't retrieve additional data element, is the component loaded?",
      );
    } else {
      return element;
    }
  } else {
    return element;
  }
};

export default additionalData;
