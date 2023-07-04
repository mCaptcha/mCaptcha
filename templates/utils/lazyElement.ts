// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

class LazyElement {
  id: string;
  element: HTMLElement;
  constructor(id: string) {
    this.id = id;
  }

  get(): HTMLElement {
    if (this.element === null || this.element === undefined) {
      const element = document.getElementById(this.id);
      if (element === null || element === undefined) {
        throw new Error(`Element ${this.id} is undefined`);
      } else {
        this.element = element;
      }
    }
    return this.element;
  }
}

export default LazyElement;
