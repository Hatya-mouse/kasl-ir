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

macro_rules! apply_ibinop {
    ($op:expr, $l:expr, $r:expr, $unsigned:ty) => {
        match $op {
            IntBinOp::Add => Some($l.wrapping_add($r)),
            IntBinOp::Sub => Some($l.wrapping_sub($r)),
            IntBinOp::Mul => Some($l.wrapping_mul($r)),
            IntBinOp::Div => ($r != 0).then(|| $l.wrapping_div($r)),
            IntBinOp::SRem => ($r != 0).then(|| $l.wrapping_rem($r)),
            IntBinOp::IShL => Some($l.wrapping_shl($r as u32)),
            IntBinOp::SShR => Some($l.wrapping_shr($r as u32)),
            IntBinOp::UShR => Some(($l as $unsigned).wrapping_shr($r as u32) as _),
            IntBinOp::Min => Some($l.min($r)),
            IntBinOp::Max => Some($l.max($r)),
            IntBinOp::BAnd => Some($l & $r),
            IntBinOp::BOr => Some($l | $r),
            IntBinOp::BXor => Some($l ^ $r),
            IntBinOp::BNand => Some(!($l & $r)),
            IntBinOp::BNor => Some(!($l | $r)),
            IntBinOp::BXnor => Some(!($l ^ $r)),
        }
    };
}

macro_rules! apply_iunaryop {
    ($op:expr, $v:expr) => {
        match $op {
            IntUnaryOp::Abs => $v.wrapping_abs(),
            IntUnaryOp::Sgn => $v.signum(),
            IntUnaryOp::Neg => $v.wrapping_neg(),
            IntUnaryOp::BNot => !$v,
        }
    };
}

macro_rules! apply_fbinop {
    ($op:expr, $l:expr, $r:expr) => {
        match $op {
            FloatBinOp::Add => $l + $r,
            FloatBinOp::Sub => $l - $r,
            FloatBinOp::Mul => $l * $r,
            FloatBinOp::Div => $l / $r,
            FloatBinOp::Rem => $l % $r,
            FloatBinOp::Pow => $l.powf($r),
            FloatBinOp::Atan2 => $l.atan2($r),
            FloatBinOp::Log => $l.log($r),
            FloatBinOp::Min => $l.min($r),
            FloatBinOp::Max => $l.max($r),
        }
    };
}

macro_rules! apply_funaryop {
    ($op:expr, $v:expr) => {
        match $op {
            FloatUnaryOp::Abs => $v.abs(),
            FloatUnaryOp::Sgn => $v.signum(),
            FloatUnaryOp::Neg => -$v,
            FloatUnaryOp::Floor => $v.floor(),
            FloatUnaryOp::Ceil => $v.ceil(),
            FloatUnaryOp::Round => $v.round(),
            FloatUnaryOp::Sin => $v.sin(),
            FloatUnaryOp::Cos => $v.cos(),
            FloatUnaryOp::Tan => $v.tan(),
            FloatUnaryOp::Asin => $v.asin(),
            FloatUnaryOp::Acos => $v.acos(),
            FloatUnaryOp::Atan => $v.atan(),
            FloatUnaryOp::Exp => $v.exp(),
            FloatUnaryOp::Log10 => $v.log10(),
            FloatUnaryOp::Log2 => $v.log2(),
            FloatUnaryOp::Sqrt => $v.sqrt(),
        }
    };
}
