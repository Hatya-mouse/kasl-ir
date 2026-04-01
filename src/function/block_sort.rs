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

use crate::{Block, Function};
use std::collections::HashSet;

impl Function {
    pub fn sorted_blocks(&self) -> Vec<Block> {
        let mut visited: HashSet<Block> = HashSet::new();
        let mut post_order: Vec<Block> = Vec::new();

        // Sort the blocks using depth first search
        if let Some(entry) = self.entry_block() {
            self.dfs(entry, &mut visited, &mut post_order);
        }

        post_order.into_iter().rev().collect()
    }

    fn dfs(&self, block: Block, visited: &mut HashSet<Block>, post_order: &mut Vec<Block>) {
        visited.insert(block);

        if let Some(block_data) = self.get_block(&block) {
            // Get the next block from the last instruction
            for next_block in block_data.get_successors() {
                if !visited.contains(&next_block) {
                    self.dfs(next_block, visited, post_order);
                }
            }
        }

        post_order.push(block);
    }
}
