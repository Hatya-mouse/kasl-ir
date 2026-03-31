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

/// Defines unary operations for integer values.
pub enum IntUnaryOp {
    /// Absolute value of the operand.
    Abs,
    /// Signum operation, which returns -1 for negative values, 0 for zero, and 1 for positive values.
    Sgn,
    /// Negation operation.
    Neg,
    /// Bitwise not operation.
    BNot,
}

impl Display for IntUnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntUnaryOp::Abs => write!(f, "abs"),
            IntUnaryOp::Sgn => write!(f, "sgn"),
            IntUnaryOp::Neg => write!(f, "neg"),
            IntUnaryOp::BNot => write!(f, "bnot"),
        }
    }
}

/// Defines unary operations for floating-point values.
pub enum FloatUnaryOp {
    /// Absolute value of the operand.
    Abs,
    /// Signum operation, which returns -1.0 for negative values, 0.0 for zero, and 1.0 for positive values.
    Sgn,
    /// Negation operation.
    Neg,
    /// Floor operation.
    Floor,
    /// Ceiling operation.
    Ceil,
    /// Rounding operation.
    Round,
    /// Calculates the sine of the operand.
    Sin,
    /// Calculates the cosine of the operand.
    Cos,
    /// Calculates the tangent of the operand.
    Tan,
    /// Calculates the arcsine of the operand.
    Asin,
    /// Calculates the arccosine of the operand.
    Acos,
    /// Calculates the arctangent of the operand.
    Atan,
    /// Calculates the exponential of the operand.
    Exp,
    /// Calculates the natural logarithm of the operand.
    Log,
    /// Calculates the square root of the operand.
    Sqrt,
}

impl Display for FloatUnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatUnaryOp::Abs => write!(f, "abs"),
            FloatUnaryOp::Sgn => write!(f, "sgn"),
            FloatUnaryOp::Neg => write!(f, "neg"),
            FloatUnaryOp::Floor => write!(f, "floor"),
            FloatUnaryOp::Ceil => write!(f, "ceil"),
            FloatUnaryOp::Round => write!(f, "round"),
            FloatUnaryOp::Sin => write!(f, "sin"),
            FloatUnaryOp::Cos => write!(f, "cos"),
            FloatUnaryOp::Tan => write!(f, "tan"),
            FloatUnaryOp::Asin => write!(f, "asin"),
            FloatUnaryOp::Acos => write!(f, "acos"),
            FloatUnaryOp::Atan => write!(f, "atan"),
            FloatUnaryOp::Exp => write!(f, "exp"),
            FloatUnaryOp::Log => write!(f, "log"),
            FloatUnaryOp::Sqrt => write!(f, "sqrt"),
        }
    }
}
