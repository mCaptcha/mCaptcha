// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

const isNumber = (value: string|number): boolean => {
  value = value.toString();
  return /^\d+$/.test(value);
};

export default isNumber;
