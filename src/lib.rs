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

//! Intermediate representation (IR) for the KASL compiler.

mod block;
mod builder;
mod function;
mod inst;
mod optimization;
mod types;
mod value;

pub use block::{Block, BlockData};
pub use builder::{IRBuilder, InstBuilder};
pub use function::Function;
pub use inst::{FloatBinOp, FloatCmp, FloatUnaryOp, Inst, IntBinOp, IntCmp, IntUnaryOp};
pub use optimization::Optimizer;
pub use types::IRType;
pub use value::{Const, Offset, ResolveOffset, Value, Variable};
