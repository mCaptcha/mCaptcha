// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

class CopyIcon {
  copyIcon: HTMLElement;
  copyDoneIconClass: string;
  writeText: string;

  constructor(
    writeText: string,
    copyIcon: HTMLElement,
    copyDoneIconClass: string
  ) {
    this.copyIcon = copyIcon;
    this.copyDoneIconClass = copyDoneIconClass;
    this.writeText = writeText;

    this.__registerHandlers();
  }

  __registerHandlers(): void {
    this.copyIcon.addEventListener("click", (e) => this.copySitekey(e));
  }

  /*
   * Copy secret to clipboard
   */
  async copySitekey(e: Event): Promise<void> {
    const image = <HTMLElement>e.target;
    const copyDoneIcon = <HTMLElement>(
      image.parentElement.querySelector(`.${this.copyDoneIconClass}`)
    );
    await navigator.clipboard.writeText(this.writeText);
    image.style.display = "none";
    copyDoneIcon.style.display = "block";
    setTimeout(() => {
      copyDoneIcon.style.display = "none";
      image.style.display = "block";
    }, 1200);
  }
}

export default CopyIcon;
