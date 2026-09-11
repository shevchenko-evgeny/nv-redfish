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

use std::error::Error as StdError;
use std::sync::Arc;

use regex::RegexSet;
pub use serde_json::Value;

///Function to transform JSON to correct RedFish object
///right before NavPropery deserialization
pub type InflightPatchFn = Arc<dyn Fn(Value) -> Value + Sync + Send>;

pub(crate) struct InflightPatch {
    pub priority: usize,
    pub oid_predicate: String,
    pub patch: InflightPatchFn,
}

// impl InflightPatch {
//     pub(crate) fn new(priority: usize, oid_predicate: &str, patch: InflightPatchFn) -> Self {
//         Self { priority, oid_predicate: oid_predicate.to_string(),  patch }
//     }
// }
#[derive(Default)]
pub struct InflightPatchRegistry {
    patches: Vec<InflightPatch>,
    regex_set: RegexSet,
}

impl InflightPatchRegistry {
    pub fn new(mut patches: Vec<InflightPatch>) -> Result<Self, Box<dyn StdError>> {
        patches.sort_by_key(|e| e.priority);
        let expressions: Vec<&str> = patches.iter().map(|p| p.oid_predicate.as_str()).collect();
        let regex_set = RegexSet::new(expressions)?;
        Ok(Self { patches, regex_set })
    }

    pub fn patch(&self, oid: &str, json: Value) -> Value {
        self.regex_set
            .matches(oid)
            .into_iter()
            .map(|i| self.patches[i].patch.clone())
            .fold(json, |i, f| f(i))
    }
}
