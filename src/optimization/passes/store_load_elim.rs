//
//  Copyright 2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::{Inst, Optimizer};
use std::collections::HashMap;

impl Optimizer {
    /// Eliminates a `Store` immediately followed by a `Load` from the same pointer and offset.
    pub(in crate::optimization) fn elim_store_load(&mut self, insts: Vec<Inst>) -> Vec<Inst> {
        let mut new_insts = Vec::with_capacity(insts.len());
        // A map to store the values being stored at specific pointers and offsets
        let mut memory_map = HashMap::new();

        for inst in insts {
            match &inst {
                Inst::Store {
                    src,
                    dst_ptr,
                    dst_offset,
                    ..
                } => {
                    // Record the value being stored at the given pointer and offset
                    memory_map.insert((*dst_ptr, dst_offset.clone()), *src);
                    new_insts.push(inst);
                }
                Inst::Load {
                    src_ptr,
                    src_offset,
                    dst,
                    ..
                } => {
                    if let Some(original_value) = memory_map.get(&(*src_ptr, src_offset.clone())) {
                        // If the load is from the same pointer and offset as a previous store, replace it with the stored value
                        self.value_replace_map.insert(*dst, *original_value);
                    } else {
                        new_insts.push(inst);
                    }
                }
                _ => new_insts.push(inst),
            }
        }

        new_insts
    }
}
