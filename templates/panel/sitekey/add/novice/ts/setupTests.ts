// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

export { trim, fillDescription } from "../../advance/ts/setupTests";

const fillField = (id: string, value: number | string) => {
  const inputElement = <HTMLInputElement>document.getElementById(id);
  inputElement.value = value.toString();
};

/** Fill peak sustainable traffic in add captcha form */
export const fillPeakSustainable = (traffic: number | string): void =>
  fillField("peak_sustainable_traffic", traffic);

/** Fill average traffic in add captcha form */
export const fillAvgTraffic = (traffic: number | string): void =>
  fillField("avg_traffic", traffic);

/** Fill broke_my_site_traffic in add captcha form */
export const fillBrokemySite = (traffic: number | string): void =>
  fillField("broke_my_site_traffic", traffic);

export const getAddForm = (): string =>
  `
<form class="sitekey-form" action="/api/v1/mcaptcha/levels/add" method="post">
  <h1 class="form__title">
    Add Sitekey
  </h1>
  <label class="sitekey-form__label" for="description">
    Description
    <input
      class="sitekey-form__input"
      type="text"
      name="description"
      id="description"
      required=""
      
    >
  </label>

    <label class="sitekey-form__label" for="avg_traffic">
		Average Traffic of your website
	  <input
		class="sitekey-form__input"
		type="number"
		name="avg_traffic"
		id="avg_traffic"
		required
		value="<.= avg_traffic .>"
	  />
	</label>

    <label class="sitekey-form__label" for="avg_traffic">
		Maximum traffic that your website can handle
	  <input
		class="sitekey-form__input"
		type="number"
		name="peak_sustainable_traffic"
		id="peak_sustainable_traffic"
		required
		value="<.= peak_sustainable_traffic .>"
	  />
	</label>

    <label class="sitekey-form__label" for="avg_traffic">
		Traffic that broke your website(Optional)
	  <input
		class="sitekey-form__input"
		type="number"
		name="broke_my_site_traffic"
		id="broke_my_site_traffic"
		value="<.= broke_my_site_traffic .>"
	  />
	</label>
  <button class="sitekey-form__submit" type="submit">Submit</button>
</form>
`;
