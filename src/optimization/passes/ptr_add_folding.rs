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

use crate::{Inst, Offset, Optimizer};

impl Optimizer {
    /// Fold consecutive pointer additions into a single addition.
    pub(in crate::optimization) fn fold_ptr_add(&self, insts: Vec<Inst>) -> Vec<Inst> {
        let mut new_insts = Vec::with_capacity(insts.len());
        let mut iter = insts.into_iter().peekable();

        while let Some(inst) = iter.next() {
            if let Inst::PtrAdd {
                ptr: ptr1,
                offset: offset1,
                dst: dst1,
            } = &inst
            {
                match iter.peek() {
                    Some(Inst::PtrAdd {
                        ptr: ptr2,
                        offset: offset2,
                        dst: dst2,
                    }) => {
                        if ptr1 == ptr2
                            && dst1 == dst2
                            && let Some(combined) = combine_offset(offset1, offset2)
                        {
                            new_insts.push(Inst::PtrAdd {
                                ptr: *ptr1,
                                offset: combined,
                                dst: *dst1,
                            });
                            iter.next();
                            continue;
                        }
                    }

                    Some(Inst::Load {
                        ty,
                        src_ptr: ptr2,
                        src_offset: offset2,
                        dst,
                    }) => {
                        if ptr1 == ptr2
                            && offset1 == offset2
                            && let Some(combined) = combine_offset(offset1, offset2)
                        {
                            new_insts.push(Inst::Load {
                                ty: *ty,
                                src_ptr: *ptr1,
                                src_offset: combined,
                                dst: *dst,
                            });
                            iter.next();
                            continue;
                        }
                    }

                    _ => {}
                }
            }

            new_insts.push(inst);
        }

        new_insts
    }
}

fn combine_offset(offset1: &Offset, offset2: &Offset) -> Option<Offset> {
    match (offset1, offset2) {
        (Offset::PointerScaled(s1), Offset::PointerScaled(s2)) => {
            Some(Offset::PointerScaled(s1 + s2))
        }
        (Offset::Immediate(i1), Offset::Immediate(i2)) => Some(Offset::Immediate(i1 + i2)),
        _ => None,
    }
}
