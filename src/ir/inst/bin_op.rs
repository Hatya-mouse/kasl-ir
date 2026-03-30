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
