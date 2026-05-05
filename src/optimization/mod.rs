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

mod passes;
mod value_replacement;

use crate::{Function, Value};
use std::collections::HashMap;

#[derive(Default)]
pub struct Optimizer {
    /// Value replacement map for tracking value replacements occured during optimization.
    value_replace_map: HashMap<Value, Value>,
}

impl Optimizer {
    pub fn optimize(&mut self, mut func: Function) -> Function {
        for block in func.blocks.values_mut() {
            let insts = std::mem::take(&mut block.insts);
            let insts = passes::store_load_elim::run(insts, &mut self.value_replace_map);
            let insts = passes::ptr_add_folding::run(insts);
            block.insts = insts;
        }

        for block in func.blocks.values_mut() {
            self.replace_values_in_block(block);
        }
        func
    }
}
