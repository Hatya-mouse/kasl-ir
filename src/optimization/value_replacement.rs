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

use crate::{BlockData, Inst, Value, optimization::Optimizer};

impl Optimizer {
    pub(super) fn replace_values_in_block(&self, block: &mut BlockData) {
        for inst in &mut block.insts {
            self.replace_values_in_inst(inst);
        }
    }

    fn replace_values_in_inst(&self, inst: &mut Inst) {
        // I know that I can combine the implementation for instructions which has the same operand pattern,
        // but in case of changing the instruction format for them in the future, I keep the implementation
        // for each instrctions separate
        match inst {
            Inst::Alloc {
                size: _,
                align: _,
                dst,
            } => self.replace_if_possible(dst),
            Inst::Assign { var: _, src } => self.replace_if_possible(src),
            Inst::Brif {
                cond,
                then_args,
                else_args,
                ..
            } => {
                self.replace_if_possible(cond);
                for arg in then_args {
                    self.replace_if_possible(arg);
                }
                for arg in else_args {
                    self.replace_if_possible(arg);
                }
            }
            Inst::Const { value: _, dst } => {
                self.replace_if_possible(dst);
            }
            Inst::FBinOp {
                op: _,
                lhs,
                rhs,
                dst,
            } => {
                self.replace_if_possible(lhs);
                self.replace_if_possible(rhs);
                self.replace_if_possible(dst);
            }
            Inst::FCmp {
                cmp: _,
                lhs,
                rhs,
                dst,
            } => {
                self.replace_if_possible(lhs);
                self.replace_if_possible(rhs);
                self.replace_if_possible(dst);
            }
            Inst::FResize {
                src,
                dst_ty: _,
                dst,
            } => {
                self.replace_if_possible(src);
                self.replace_if_possible(dst);
            }
            Inst::FToI {
                src,
                dst_ty: _,
                dst,
            } => {
                self.replace_if_possible(src);
                self.replace_if_possible(dst);
            }
            Inst::FUnaryOp {
                op: _,
                operand,
                dst,
            } => {
                self.replace_if_possible(operand);
                self.replace_if_possible(dst);
            }
            Inst::IBinOp {
                op: _,
                lhs,
                rhs,
                dst,
            } => {
                self.replace_if_possible(lhs);
                self.replace_if_possible(rhs);
                self.replace_if_possible(dst);
            }
            Inst::ICmp {
                cmp: _,
                lhs,
                rhs,
                dst,
            } => {
                self.replace_if_possible(lhs);
                self.replace_if_possible(rhs);
                self.replace_if_possible(dst);
            }
            Inst::ICmpImm {
                cmp: _,
                lhs,
                rhs: _,
                dst,
            } => {
                self.replace_if_possible(lhs);
                self.replace_if_possible(dst);
            }
            Inst::IResize {
                src,
                dst_ty: _,
                dst,
            } => {
                self.replace_if_possible(src);
                self.replace_if_possible(dst);
            }
            Inst::IToF {
                src,
                dst_ty: _,
                dst,
            } => {
                self.replace_if_possible(src);
                self.replace_if_possible(dst);
            }
            Inst::IUnaryOp {
                op: _,
                operand,
                dst,
            } => {
                self.replace_if_possible(operand);
                self.replace_if_possible(dst);
            }
            Inst::Jump { block: _, args } => {
                for arg in args {
                    self.replace_if_possible(arg);
                }
            }
            Inst::Load {
                ty: _,
                src_ptr,
                src_offset: _,
                dst,
            } => {
                self.replace_if_possible(src_ptr);
                self.replace_if_possible(dst);
            }
            Inst::LoadVar { var: _, dst } => {
                self.replace_if_possible(dst);
            }
            Inst::Memcpy {
                size: _,
                src_ptr,
                dst_ptr,
            } => {
                self.replace_if_possible(src_ptr);
                self.replace_if_possible(dst_ptr);
            }
            Inst::Memset {
                size: _,
                value: _,
                dst_ptr,
            } => {
                self.replace_if_possible(dst_ptr);
            }
            Inst::PtrAdd {
                ptr,
                offset: _,
                dst,
            } => {
                self.replace_if_possible(ptr);
                self.replace_if_possible(dst);
            }
            Inst::Return { vals } => {
                for val in vals {
                    self.replace_if_possible(val);
                }
            }
            Inst::Select {
                cond,
                then_val,
                else_val,
                dst,
            } => {
                self.replace_if_possible(cond);
                self.replace_if_possible(then_val);
                self.replace_if_possible(else_val);
                self.replace_if_possible(dst);
            }
            Inst::Store {
                src,
                dst_ptr,
                dst_offset: _,
            } => {
                self.replace_if_possible(src);
                self.replace_if_possible(dst_ptr);
            }
        }
    }

    fn replace_if_possible(&self, value: &mut Value) {
        if let Some(replacement) = self.value_replace_map.get(value) {
            *value = *replacement;
        }
    }
}
