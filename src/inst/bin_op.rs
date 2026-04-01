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

use std::fmt::Display;

/// Defines binary operations for integer values.
pub enum IntBinOp {
    /// Addition operation.
    Add,
    /// Subtraction operation.
    Sub,
    /// Multiplication operation.
    Mul,
    /// Division operation.
    Div,
    /// Signed remainder operation.
    SRem,
    /// Shift left operation.
    IShL,
    /// Signed shift right operation. Also called arithmetic shift.
    SShR,
    /// Unsigned shift right operation. Also called logical shift.
    UShR,
    /// Returns the minimum of the first operand and the second operand.
    Min,
    /// Returns the maximum of the first operand and the second operand.
    Max,
    /// Bitwise AND.
    BAnd,
    /// Bitwise OR.
    BOr,
    /// Bitwise XOR.
    BXor,
    /// Bitwise NAND.
    BNand,
    /// Bitwise NOR.
    BNor,
    /// Bitwise XNOR.
    BXnor,
}

impl Display for IntBinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntBinOp::Add => write!(f, "add"),
            IntBinOp::Sub => write!(f, "sub"),
            IntBinOp::Mul => write!(f, "mul"),
            IntBinOp::Div => write!(f, "div"),
            IntBinOp::SRem => write!(f, "srem"),
            IntBinOp::IShL => write!(f, "ishl"),
            IntBinOp::SShR => write!(f, "sshr"),
            IntBinOp::UShR => write!(f, "ushr"),
            IntBinOp::Min => write!(f, "min"),
            IntBinOp::Max => write!(f, "max"),
            IntBinOp::BAnd => write!(f, "band"),
            IntBinOp::BOr => write!(f, "bor"),
            IntBinOp::BXor => write!(f, "bxor"),
            IntBinOp::BNand => write!(f, "bnand"),
            IntBinOp::BNor => write!(f, "bnor"),
            IntBinOp::BXnor => write!(f, "bxnor"),
        }
    }
}

/// Defines binary operations for floating-point values.
pub enum FloatBinOp {
    /// Addition operation.
    Add,
    /// Subtraction operation.
    Sub,
    /// Multiplication operation.
    Mul,
    /// Division operation.
    Div,
    /// Remainder operation.
    Rem,
    /// Raises the first operand to the power of the second operand.
    Pow,
    /// Returns the atan2 of the first operand and the second operand.
    Atan2,
    /// Returns the minimum of the first operand and the second operand.
    Min,
    /// Returns the maximum of the first operand and the second operand.
    Max,
}

impl Display for FloatBinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatBinOp::Add => write!(f, "add"),
            FloatBinOp::Sub => write!(f, "sub"),
            FloatBinOp::Mul => write!(f, "mul"),
            FloatBinOp::Div => write!(f, "div"),
            FloatBinOp::Rem => write!(f, "rem"),
            FloatBinOp::Pow => write!(f, "pow"),
            FloatBinOp::Atan2 => write!(f, "atan2"),
            FloatBinOp::Min => write!(f, "min"),
            FloatBinOp::Max => write!(f, "max"),
        }
    }
}
