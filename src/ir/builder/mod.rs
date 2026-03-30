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

mod inst_builder;

pub use inst_builder::InstBuilder;

use crate::ir::{Block, BlockData, Inst, Value, Variable, function::Function};

#[derive(Default)]
pub struct IRBuilder {
    /// The function which is being built.
    function: Function,
    /// The current block being built.
    current_block: Option<Block>,

    /// The next value ID.
    next_val_id: u32,
    /// The next variable ID.
    next_var_id: u32,
    /// The next block ID.
    next_block_id: u32,
}

impl IRBuilder {
    // --- ID GENERATION ---
    fn gen_val_id(&mut self) -> Value {
        let id = Value(self.next_val_id);
        self.next_val_id += 1;
        id
    }

    fn gen_var_id(&mut self) -> Variable {
        let id = Variable(self.next_var_id);
        self.next_var_id += 1;
        id
    }

    fn gen_block_id(&mut self) -> Block {
        let id = Block(self.next_block_id);
        self.next_block_id += 1;
        id
    }

    // --- BLOCK OPERATION ---

    /// Creates a new block and returns the newly created block.
    pub fn create_block(&mut self) -> Block {
        let block = self.gen_block_id();
        self.function.blocks.insert(block, BlockData::default());
        block
    }

    /// Sets the current block to the given block.
    pub fn set_current_block(&mut self, block: Block) {
        self.current_block = Some(block);
    }

    /// Sets the entry block of the function.
    pub fn set_entry_block(&mut self, block: Block) {
        self.function.entry_block = Some(block);
    }

    // --- VARIABLE OPERATION ---

    /// Creates a new variable and returns the newly created variable.
    pub fn create_variable(&mut self) -> Variable {
        self.gen_var_id()
    }

    // --- INSTRUCTION UTILITY ---

    /// Adds an instruction to the current block.
    fn push_inst(&mut self, inst: Inst) {
        if let Some(target_block) = self
            .current_block
            .as_ref()
            .and_then(|block| self.function.get_block_mut(block))
        {
            target_block.insts.push(inst);
        }
    }
}
