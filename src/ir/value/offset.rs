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

use std::fmt::Display;

/// An offset used in memory access instructions.
pub enum Offset {
    /// Physical byte count.
    Immediate(u32),
    /// Pointer size, mutiplied by the given factor.
    PointerScaled(u32),
}

impl Offset {
    /// Returns a new offset with the immediate value of zero.
    pub fn zero() -> Self {
        Offset::Immediate(0)
    }
}

/// A trait to resolve an `Offset` to an actual byte offset, given the pointer size of the target architecture.
pub trait ResolveOffset {
    fn resolve(&self, ptr_size: u32) -> u32;
}

impl ResolveOffset for Offset {
    fn resolve(&self, ptr_size: u32) -> u32 {
        match self {
            Offset::Immediate(n) => *n,
            Offset::PointerScaled(n) => *n * ptr_size,
        }
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Offset::Immediate(offset) => write!(f, "imm({})", offset),
            Offset::PointerScaled(scale) => write!(f, "ptr*{}", scale),
        }
    }
}
