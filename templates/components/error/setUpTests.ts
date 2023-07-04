// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import * as e from "./index";

const setup = (): HTMLElement => {
  const x = document.createElement("div");
  x.id = e.ERR_CONTAINER_ID;
  return x;
};

export default setup;
