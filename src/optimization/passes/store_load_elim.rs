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

impl Optimizer {
    /// Eliminates a `Store` immediately followed by a `Load` from the same pointer and offset.
    pub(in crate::optimization) fn elim_store_load(&mut self, insts: Vec<Inst>) -> Vec<Inst> {
        let mut new_insts = Vec::with_capacity(insts.len());
        let mut iter = insts.into_iter().peekable();

        while let Some(inst) = iter.next() {
            if let Inst::Store {
                src,
                dst_ptr,
                dst_offset,
            } = &inst
                && let Some(Inst::Load {
                    src_ptr,
                    src_offset,
                    dst,
                    ..
                }) = iter.peek()
                && dst_ptr == src_ptr
                && dst_offset == src_offset
            {
                // Replace the loaded value with the original value
                self.value_replace_map.insert(*dst, *src);
                iter.next();
            }
            // Add the store instruction even if the load is eliminated as it may have some side effects
            new_insts.push(inst);
        }

        new_insts
    }
}
