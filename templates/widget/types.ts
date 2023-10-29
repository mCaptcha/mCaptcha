// Copyright © 2021 Aravinth Manivnanan <realaravinth@batsense.net>.
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

export type Work = {
  result: string;
  nonce: number;
  string: string;
  key: string;
  time: number;
  worker_type: string;
};

export type SubmitWork = {
  time: number;
  worker_type: string;
  result: string;
  nonce: number;
};

export type WasmWork = {
  result: string;
  nonce: number;
};

export type ServiceWorkerWork = {
  work: SubmitWork;
};

export type PoWConfig = {
  string: string;
  difficulty_factor: number;
  salt: string;
  max_recorded_nonce: number;
};

export type Token = {
  token: string;
};

export type ServiceWorkerMessage =
  | { type: "work"; value: ServiceWorkerWork }
  | { type: "progress"; nonce: number };
