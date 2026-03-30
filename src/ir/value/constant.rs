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

use crate::ir::IRType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Const {
    /// A constant 8-bit integer value.
    I8(i8),
    /// A constant 16-bit integer value.
    I16(i16),
    /// A constant 32-bit integer value.
    I32(i32),
    /// A constant 64-bit integer value.
    I64(i64),
    /// A constant 32-bit floating-point value.
    F32(f32),
    /// A constant 64-bit floating-point value.
    F64(f64),
    /// A constant pointer-size integer value.
    Ptr(i64),
}

impl Const {
    /// Returns the type of the constant value.
    pub(crate) fn get_type(&self) -> IRType {
        match self {
            Const::I8(_) => IRType::I8,
            Const::I16(_) => IRType::I16,
            Const::I32(_) => IRType::I32,
            Const::I64(_) => IRType::I64,
            Const::F32(_) => IRType::F32,
            Const::F64(_) => IRType::F64,
            Const::Ptr(_) => IRType::Ptr,
        }
    }
}
