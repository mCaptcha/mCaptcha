// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
