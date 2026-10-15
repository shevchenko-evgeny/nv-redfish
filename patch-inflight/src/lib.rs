// SPDX-FileCopyrightText: Copyright (c) 2025-2026 MIRANTIS, INC. & AFFILIATES. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub(crate) mod fixes;
pub(crate) mod patch_registry;
use fixes::fix_ntp_null_elements;
use patch_registry::{InflightPatch, InflightPatchRegistry};
use serde_json::Value;
use std::sync::{Arc, LazyLock};

pub(crate) static TRASFORMATOR: LazyLock<InflightPatchRegistry> = LazyLock::new(|| {
    let fix_ntp_null = InflightPatch {
        priority: 1000,
        oid_predicate: r"^/redfish/v1/Managers/[^/]+/NetworkProtocol/?$".into(),
        patch: Arc::new(fix_ntp_null_elements),
    };
    InflightPatchRegistry::new(vec![fix_ntp_null]).unwrap_or_default()});

pub fn patch_inflight(mut v: Value) -> Value {
    let oid = v
        .as_object()
        .and_then(|o| o.get("@odata.id"))
        .and_then(|s| s.as_str())
        .map(str::to_owned);

    if let Some(oid) = oid {
        v = TRASFORMATOR.patch(&oid, v);
    }
    v
}
