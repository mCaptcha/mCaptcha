/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
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
import {
  getAddForm,
  fillAvgTraffic,
  fillDescription,
  fillPeakSustainable,
  fillBrokemySite,
} from "./setupTests";
import setup from "../../../../../components/error/setUpTests";

export const break_my_site_name = "traffic that broke your website";
export const avg_traffic_name = "average";
export const peak_traffic_name = "maximum traffic your website can handle";

beforeEach(() => {
  document.body.innerHTML = getAddForm();
  document.body.appendChild(setup());
});

afterEach(() => {
  document.body.replaceWith(document.createElement("body"));
});

const checkEmpty = (e: Error, name: string) => {
  expect(e.message.includes(name)).toBeTruthy();
  expect(e.message.includes(name)).toBeTruthy();
};

it("empty description", () => {
  const form = <HTMLFormElement>document.querySelector("form");

  fillAvgTraffic(1);
  fillPeakSustainable(2);
  try {
    form.submit();
  } catch (e) {
    checkEmpty(e, "description");
  }
});

it("empty average traffic", () => {
  const form = <HTMLFormElement>document.querySelector("form");

  fillDescription("foo");
  fillPeakSustainable(2);
  try {
    form.submit();
  } catch (e) {
    checkEmpty(e, avg_traffic_name);
  }
});

it("empty peak traffic", () => {
  const form = <HTMLFormElement>document.querySelector("form");
  fillDescription("foo");
  fillAvgTraffic(1);
  try {
    form.submit();
  } catch (e) {
    checkEmpty(e, peak_traffic_name);
  }
});

const checkNan = (e: Error, name: string) => {
  expect(e.message.includes(`${name} must be a number`)).toBeTruthy();
};

it("NAN peak traffic", () => {
  const form = <HTMLFormElement>document.querySelector("form");
  fillDescription("foo");
  fillAvgTraffic(1);
  fillPeakSustainable("foo");
  try {
    form.submit();
  } catch (e) {
    checkNan(e, peak_traffic_name);
  }
});

it("NAN Avg Traffic traffic", () => {
  const form = <HTMLFormElement>document.querySelector("form");
  fillDescription("foo");
  fillAvgTraffic("foo");
  fillPeakSustainable(1);
  try {
    form.submit();
  } catch (e) {
    checkNan(e, avg_traffic_name);
  }
});

it("NAN Break my site Traffic traffic", () => {
  const form = <HTMLFormElement>document.querySelector("form");
  fillDescription("foo");
  fillAvgTraffic(1);
  fillPeakSustainable(1);
  fillBrokemySite("foo");
  try {
    form.submit();
  } catch (e) {
    checkNan(e, break_my_site_name);
  }
});

const GetMustB = (lhs: string, rhs: string) =>
  `${lhs} must be greater than ${rhs}`;
const CheckMustBeGreater = (e: Error, lhs: string, rhs: string) => {
  const msg = GetMustB(lhs, rhs);
  expect(e.message.includes(msg)).toBeTruthy();
};

it(GetMustB(break_my_site_name, peak_traffic_name), () => {
  const form = <HTMLFormElement>document.querySelector("form");
  fillDescription("foo");
  fillAvgTraffic(100);
  fillPeakSustainable(1000);
  fillBrokemySite(999);
  try {
    form.submit();
  } catch (e) {
    CheckMustBeGreater(e, break_my_site_name, peak_traffic_name);
  }

  fillBrokemySite(1000);
  try {
    form.submit();
  } catch (e) {
    CheckMustBeGreater(e, break_my_site_name, peak_traffic_name);
  }
});

it(GetMustB(peak_traffic_name, avg_traffic_name), () => {
  const form = <HTMLFormElement>document.querySelector("form");
  fillDescription("foo");
  fillAvgTraffic(1000);
  fillPeakSustainable(999);
  try {
    form.submit();
  } catch (e) {
    CheckMustBeGreater(e, peak_traffic_name, avg_traffic_name);
  }

  fillPeakSustainable(1000);
  try {
    form.submit();
  } catch (e) {
    CheckMustBeGreater(e, peak_traffic_name, avg_traffic_name);
  }
});
