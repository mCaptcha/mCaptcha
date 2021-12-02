/*
 * Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
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
import prove from "./prove";
import { PoWConfig, ServiceWorkerWork } from "./types";
import log from "../logger";

log.log("worker registered");
onmessage = (e) => {
  console.debug("message received at worker");
  const config: PoWConfig = e.data;

  const t0 = performance.now();
  const work = prove(config);

  const t1 = performance.now();
  const duration = t1 - t0;

  const res: ServiceWorkerWork = {
    work,
    duration,
  };

  postMessage(res);
};
