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

use crate::{Inst, Value};
use std::collections::HashMap;

/// Eliminates a `Store` immediately followed by a `Load` from the same pointer and offset.
/// The loaded value is replaced with the stored value via `replace_map`.
pub(in crate::optimization) fn run(
    insts: Vec<Inst>,
    replace_map: &mut HashMap<Value, Value>,
) -> Vec<Inst> {
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
            replace_map.insert(*dst, *src);
            iter.next();
            continue;
        }
        new_insts.push(inst);
    }

    new_insts
}
