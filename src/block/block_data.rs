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

use crate::{Block, Inst, Value, value::fmt_vals};
use std::fmt::Display;

#[derive(Default, Clone)]
pub struct BlockData {
    pub(crate) params: Vec<Value>,
    pub(crate) insts: Vec<Inst>,
}

impl BlockData {
    /// Returns the parameters of the block.
    pub fn get_params(&self) -> &[Value] {
        &self.params
    }

    /// Returns the instructions in the block.
    pub fn get_insts(&self) -> &[Inst] {
        &self.insts
    }

    /// Returns all successors of the block.
    pub fn get_successors(&self) -> Vec<Block> {
        self.insts
            .last()
            .map(|last| last.successors())
            .unwrap_or_default()
    }
}

impl Display for BlockData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "({}):", fmt_vals(&self.params))?;
        for inst in &self.insts {
            writeln!(f, "    {}", inst)?;
        }
        writeln!(f)
    }
}
