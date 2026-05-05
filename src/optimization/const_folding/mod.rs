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

#[macro_use]
mod macros;

use crate::{Const, FloatBinOp, FloatUnaryOp, Inst, IntBinOp, IntUnaryOp, Optimizer};

impl Optimizer {
    pub(super) fn fold_consts(&mut self, insts: Vec<Inst>) -> Vec<Inst> {
        let mut new_insts = Vec::with_capacity(insts.len());

        for inst in insts {
            if let Some(Inst::Const { value, dst }) = self.try_fold(&inst) {
                self.const_map.insert(dst, value);
                new_insts.push(Inst::Const { value, dst });
            } else {
                new_insts.push(inst);
            }
        }

        new_insts
    }

    /// Try to fold the given operation instruction into a constant instruction. If the folding is failed, the function will return None.
    fn try_fold(&self, inst: &Inst) -> Option<Inst> {
        match inst {
            Inst::IBinOp { op, lhs, rhs, dst } => {
                let lhs_const = *self.const_map.get(lhs)?;
                let rhs_const = *self.const_map.get(rhs)?;
                Some(Inst::Const {
                    value: fold_ibinop(op, lhs_const, rhs_const)?,
                    dst: *dst,
                })
            }
            Inst::IUnaryOp { op, operand, dst } => {
                let operand_const = *self.const_map.get(operand)?;
                Some(Inst::Const {
                    value: fold_iunaryop(op, operand_const)?,
                    dst: *dst,
                })
            }
            Inst::FBinOp { op, lhs, rhs, dst } => {
                let lhs_const = *self.const_map.get(lhs)?;
                let rhs_const = *self.const_map.get(rhs)?;
                Some(Inst::Const {
                    value: fold_fbinop(op, lhs_const, rhs_const)?,
                    dst: *dst,
                })
            }
            Inst::FUnaryOp { op, operand, dst } => {
                let operand_const = *self.const_map.get(operand)?;
                Some(Inst::Const {
                    value: fold_funaryop(op, operand_const)?,
                    dst: *dst,
                })
            }
            _ => None,
        }
    }
}

// --- CONSTANT FOLDING CALCULATION ---
// If the type of the oprands are different, the folding fails and the functions return None.

fn fold_ibinop(op: &IntBinOp, lhs: Const, rhs: Const) -> Option<Const> {
    match (lhs, rhs) {
        (Const::I8(l), Const::I8(r)) => apply_ibinop!(op, l, r, u8).map(Const::I8),
        (Const::I16(l), Const::I16(r)) => apply_ibinop!(op, l, r, u16).map(Const::I16),
        (Const::I32(l), Const::I32(r)) => apply_ibinop!(op, l, r, u32).map(Const::I32),
        (Const::I64(l), Const::I64(r)) => apply_ibinop!(op, l, r, u64).map(Const::I64),
        _ => None,
    }
}

fn fold_iunaryop(op: &IntUnaryOp, val: Const) -> Option<Const> {
    match val {
        Const::I8(v) => Some(Const::I8(apply_iunaryop!(op, v))),
        Const::I16(v) => Some(Const::I16(apply_iunaryop!(op, v))),
        Const::I32(v) => Some(Const::I32(apply_iunaryop!(op, v))),
        Const::I64(v) => Some(Const::I64(apply_iunaryop!(op, v))),
        _ => None,
    }
}

fn fold_fbinop(op: &FloatBinOp, lhs: Const, rhs: Const) -> Option<Const> {
    match (lhs, rhs) {
        (Const::F32(l), Const::F32(r)) => Some(Const::F32(apply_fbinop!(op, l, r))),
        (Const::F64(l), Const::F64(r)) => Some(Const::F64(apply_fbinop!(op, l, r))),
        _ => None,
    }
}

fn fold_funaryop(op: &FloatUnaryOp, val: Const) -> Option<Const> {
    match val {
        Const::F32(v) => Some(Const::F32(apply_funaryop!(op, v))),
        Const::F64(v) => Some(Const::F64(apply_funaryop!(op, v))),
        _ => None,
    }
}
