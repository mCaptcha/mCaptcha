// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import {LEVELS} from "./index";
import getLevelFields from "./getLevelFields";
import createError from "../../../../../../components/error/";

/**
 * Fetches level from DOM using the ID passesd and validates
 * its contents
 * */
const validateLevel = (id: number): boolean => {
  try {
    const level = getLevelFields(id);
    LEVELS.add(level);
    return true;
  } catch (e) {
    createError(e.message);
    return false;
  }
};

export default validateLevel;
