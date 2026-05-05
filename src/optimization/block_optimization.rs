//
//  Copyright 2025-2026 Shuntaro Kasatani
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

use crate::{BlockData, Inst, Offset, optimization::Optimizer};

impl Optimizer {
    pub(super) fn optimize_block(&mut self, block: &mut BlockData) {
        let mut new_insts = Vec::with_capacity(block.insts.len());
        let mut inst_iter = block.insts.iter_mut().peekable();

        while let Some(inst) = inst_iter.next() {
            match &inst {
                Inst::Store {
                    src,
                    dst_ptr,
                    dst_offset,
                } => {
                    // If the next instruction is a load from the same pointer and offset, eliminate them and replace all uses of the value with the source value of the store
                    if let Some(Inst::Load {
                        src_ptr,
                        src_offset,
                        dst,
                        ..
                    }) = inst_iter.peek()
                        && dst_ptr == src_ptr
                        && dst_offset == src_offset
                    {
                        // Do not add the store and load instruction and replace all uses of dst with src
                        self.value_replace_map.insert(*dst, *src);
                        // Consume the next instruction
                        inst_iter.next();
                        continue;
                    }
                }

                Inst::PtrAdd {
                    ptr: ptr1,
                    offset: offset1,
                    dst: dst1,
                } => match inst_iter.peek() {
                    Some(Inst::PtrAdd {
                        ptr: ptr2,
                        offset: offset2,
                        dst: dst2,
                    }) => {
                        if ptr1 == ptr2
                            && dst1 == dst2
                            && let Some(combined_offset) = combine_offset(offset1, offset2)
                        {
                            new_insts.push(Inst::PtrAdd {
                                ptr: *ptr1,
                                offset: combined_offset,
                                dst: *dst1,
                            });
                            // Consume the next instruction
                            inst_iter.next();
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
                            && let Some(combined_offset) = combine_offset(offset1, offset2)
                        {
                            // Replace the load with a load from the original pointer plus the combined offset
                            new_insts.push(Inst::Load {
                                ty: *ty,
                                src_ptr: *ptr1,
                                src_offset: combined_offset,
                                dst: *dst,
                            });
                            // Consume the next instruction
                            inst_iter.next();
                            continue;
                        }
                    }

                    _ => (),
                },

                _ => (),
            }

            new_insts.push(inst.clone());
        }
    }
}

fn combine_offset(offset1: &Offset, offset2: &Offset) -> Option<Offset> {
    match (offset1, offset2) {
        (Offset::PointerScaled(scale1), Offset::PointerScaled(scale2)) => {
            Some(Offset::PointerScaled(scale1 + scale2))
        }
        (Offset::Immediate(imm1), Offset::Immediate(imm2)) => Some(Offset::Immediate(imm1 + imm2)),
        _ => None,
    }
}
