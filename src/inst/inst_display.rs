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

use crate::{Inst, value::fmt_vals};
use std::fmt::Display;

impl Display for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Inst::Alloc { size, align, dst } => {
                write!(f, "{} = alloc size={} align={}", dst, size, align)
            }
            Inst::Load {
                ty,
                src_ptr,
                src_offset,
                dst,
            } => {
                write!(f, "{} = load ty={} {}+{}", dst, ty, src_ptr, src_offset)
            }
            Inst::Store {
                src,
                dst_ptr,
                dst_offset,
            } => {
                write!(f, "store {} -> {}+{}", src, dst_ptr, dst_offset)
            }
            Inst::Memcpy {
                size,
                src_ptr,
                dst_ptr,
            } => {
                write!(f, "memcpy size={} {} -> {}", size, src_ptr, dst_ptr)
            }
            Inst::Memset {
                size,
                value,
                dst_ptr,
            } => {
                write!(f, "memset size={} {} -> {}", size, value, dst_ptr)
            }
            Inst::Const { value, dst } => {
                write!(f, "{} = const {}", dst, value)
            }
            Inst::Assign { var, src } => {
                write!(f, "{} = {}", var, src)
            }
            Inst::LoadVar { var, dst } => {
                write!(f, "{} = load_var {}", dst, var)
            }
            Inst::Jump { block, args } => {
                write!(f, "jump {}({})", block, fmt_vals(args))
            }
            Inst::Brif {
                cond,
                then_block,
                then_args,
                else_block,
                else_args,
            } => {
                write!(
                    f,
                    "brif {} -> {}({}) else {}({})",
                    cond,
                    then_block,
                    fmt_vals(then_args),
                    else_block,
                    fmt_vals(else_args)
                )
            }
            Inst::Return { vals } => {
                write!(f, "return {}", fmt_vals(vals))
            }
            Inst::Select {
                cond,
                then_val,
                else_val,
                dst,
            } => {
                write!(
                    f,
                    "{} = select {} -> {} else {}",
                    dst, cond, then_val, else_val
                )
            }
            Inst::IResize { src, dst_ty, dst } => {
                write!(f, "{} = iresize {} dst_ty={}", dst, src, dst_ty)
            }
            Inst::FResize { src, dst_ty, dst } => {
                write!(f, "{} = fresize {} dst_ty={}", dst, src, dst_ty)
            }
            Inst::IToF { src, dst_ty, dst } => {
                write!(f, "{} = itof {} dst_ty={}", dst, src, dst_ty)
            }
            Inst::FToI { src, dst_ty, dst } => {
                write!(f, "{} = ftoi {} dst_ty={}", dst, src, dst_ty)
            }
            Inst::PtrAdd { ptr, offset, dst } => {
                write!(f, "{} = ptr_add {} {}", dst, ptr, offset)
            }
            Inst::IBinOp { op, lhs, rhs, dst } => {
                write!(f, "{} = ibop {} {}, {}", dst, op, lhs, rhs)
            }
            Inst::FBinOp { op, lhs, rhs, dst } => {
                write!(f, "{} = fbop {} {}, {}", dst, op, lhs, rhs)
            }
            Inst::IUnaryOp { op, operand, dst } => {
                write!(f, "{} = iuop {} {}", dst, op, operand)
            }
            Inst::FUnaryOp { op, operand, dst } => {
                write!(f, "{} = fuop {} {}", dst, op, operand)
            }
            Inst::ICmp { cmp, lhs, rhs, dst } => {
                write!(f, "{} = icmp {} {}, {}", dst, cmp, lhs, rhs)
            }
            Inst::FCmp { cmp, lhs, rhs, dst } => {
                write!(f, "{} = fcmp {} {}, {}", dst, cmp, lhs, rhs)
            }
            Inst::ICmpImm { cmp, lhs, rhs, dst } => {
                write!(f, "{} = icmp_imm {} {}, {}", dst, cmp, lhs, rhs)
            }
            Inst::FCmpImm { cmp, lhs, rhs, dst } => {
                write!(f, "{} = fcmp_imm {} {}, {}", dst, cmp, lhs, rhs)
            }
        }
    }
}
