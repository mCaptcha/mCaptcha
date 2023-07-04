// Copyright © 2021 Aravinth Manivnanan <realaravinth@batsense.net>.
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

import * as CONST from "../const";

import {getBaseHtml, sitekey, checkbox} from "./setupTests";
import * as TESTElements from "./setupTests";

it("const works", () => {
  const body = document.querySelector("body");
  const container = getBaseHtml();
  body.appendChild(container);
  expect(CONST.sitekey()).toBe(sitekey);
  expect(CONST.btn()).toBe(checkbox);

  // display after
  CONST.messageText().after();
  expect(TESTElements.afterMsg.style.display).toBe("block");
  expect(TESTElements.beforeMsg.style.display).toBe("none");
  expect(TESTElements.duringMsg.style.display).toBe("none");
  expect(TESTElements.errorMsg.style.display).toBe("none");

  // display before
  CONST.messageText().before();
  expect(TESTElements.afterMsg.style.display).toBe("none");
  expect(TESTElements.beforeMsg.style.display).toBe("block");
  expect(TESTElements.duringMsg.style.display).toBe("none");
  expect(TESTElements.errorMsg.style.display).toBe("none");

  // display during
  CONST.messageText().during();
  expect(TESTElements.afterMsg.style.display).toBe("none");
  expect(TESTElements.beforeMsg.style.display).toBe("none");
  expect(TESTElements.duringMsg.style.display).toBe("block");
  expect(TESTElements.errorMsg.style.display).toBe("none");

  // display error
  CONST.messageText().error();
  expect(TESTElements.afterMsg.style.display).toBe("none");
  expect(TESTElements.beforeMsg.style.display).toBe("none");
  expect(TESTElements.duringMsg.style.display).toBe("none");
  expect(TESTElements.errorMsg.style.display).toBe("block");
});
