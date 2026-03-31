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

impl Display for IntCmp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntCmp::Eq => write!(f, "eq"),
            IntCmp::Ne => write!(f, "ne"),
            IntCmp::Sgt => write!(f, "sgt"),
            IntCmp::Sge => write!(f, "sge"),
            IntCmp::Slt => write!(f, "slt"),
            IntCmp::Sle => write!(f, "sle"),
            IntCmp::Ugt => write!(f, "ugt"),
            IntCmp::Uge => write!(f, "uge"),
            IntCmp::Ult => write!(f, "ult"),
            IntCmp::Ule => write!(f, "ule"),
        }
    }
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

impl Display for FloatCmp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatCmp::Eq => write!(f, "eq"),
            FloatCmp::Ne => write!(f, "ne"),
            FloatCmp::Gt => write!(f, "gt"),
            FloatCmp::Ge => write!(f, "ge"),
            FloatCmp::Lt => write!(f, "lt"),
            FloatCmp::Le => write!(f, "le"),
        }
    }
}
