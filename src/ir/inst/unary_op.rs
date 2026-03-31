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
    /// Calculates the exponential of the operand.
    Exp,
    /// Calculates the natural logarithm of the operand.
    Log,
    /// Calculates the square root of the operand.
    Sqrt,
}
