// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import isNumber from "../../../../../../utils/isNumber";

const validateDuration = (): number => {
  const duartionElement = <HTMLInputElement>document.getElementById("duration");
  const duration = parseInt(duartionElement.value);
  if (!isNumber(duration) || Number.isNaN(duration)) {
    throw new Error("duration can contain nubers only");
  }

  if (duration <= 0) {
    throw new Error("duration must be greater than zero");
  }
  return duration;
};

export default validateDuration;
