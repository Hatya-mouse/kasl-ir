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

/// Comparison operations for integers.
pub enum IntCmp {
    /// Equal
    Eq,
    /// Not equal
    Ne,
    /// Signed greater than
    Sgt,
    /// Signed greater than or equal
    Sge,
    /// Signed less than
    Slt,
    /// Signed less than or equal
    Sle,
    /// Unsigned greater than
    Ugt,
    /// Unsigned greater than or equal
    Uge,
    /// Unsigned less than
    Ult,
    /// Unsigned less than or equal
    Ule,
}

/// Comparison operations for floating-point numbers.
pub enum FloatCmp {
    /// Equal
    Eq,
    /// Not equal
    Ne,
    /// Greater than
    Gt,
    /// Greater than or equal
    Ge,
    /// Less than
    Lt,
    /// Less than or equal
    Le,
}
