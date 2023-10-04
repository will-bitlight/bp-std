// Modern, minimalistic & standard-compliant cold wallet library.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2020-2023 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2020-2023 LNP/BP Standards Association. All rights reserved.
// Copyright (C) 2020-2023 Dr Maxim Orlovsky. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::str::FromStr;

use psbt::Psbt;

#[test]
fn pkh_0() {
    const PSBT: &str = include_str!("pkh_0.base64.psbt");
    let psbt = Psbt::from_str(PSBT).unwrap();
    Psbt::from_str(&psbt.to_string()).unwrap();
}

#[test]
fn pkh_shWpkh_0() {
    const PSBT: &str = include_str!("pkh+shWpkh_0.base64.psbt");
    let psbt = Psbt::from_str(PSBT).unwrap();
    Psbt::from_str(&psbt.to_string()).unwrap();
}

#[test]
fn all() {
    const PSBT: &str = include_str!("all.base64.psbt");
    let psbt = Psbt::from_str(PSBT).unwrap();
    Psbt::from_str(&psbt.to_string()).unwrap();
}
