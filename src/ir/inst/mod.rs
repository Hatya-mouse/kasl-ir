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

mod bin_op;
mod unary_op;

pub use bin_op::{FloatBinOp, IntBinOp};
pub use unary_op::{FloatUnaryOp, IntUnaryOp};

use crate::ir::{Block, Const, IRType, Value, value::Variable};

/// Defines the instructions in the IR.
pub enum Inst {
    /// Allocates memory on the stack with the given size and the alignment, and stores the pointer in the destination pointer.
    Alloc {
        size: u32,
        alignment: u32,
        dst: Value,
    },

    /// Loads a value from the source pointer and stores the loaded value in the destination pointer.
    Load {
        ty: IRType,
        ptr: Value,
        offset: u32,
        dst: Value,
    },

    /// Stores a src value to the destination pointer.
    Store {
        ty: IRType,
        ptr: Value,
        offset: u32,
        src: Value,
    },

    /// Copies the value stored in the source pointer to the destination pointer.
    Memcpy {
        size: Value,
        dest_ptr: Value,
        dest_offset: u32,
        src_ptr: Value,
        src_offset: u32,
    },

    /// Assigns a constant value to the value register.
    Const { value: Const, dst: Value },

    /// Assigns the value of the source register to the destination register.
    Assign { var: Variable, src: Value },

    /// Jumps to the target block with the given arguments.
    Jump { block: Block, args: Vec<Value> },

    /// Contidionally jumps to the then block if the condition is true, otherwise jumps to the else block.
    Brif {
        cond: Value,
        then_block: Block,
        then_args: Vec<Value>,
        else_block: Block,
        else_args: Vec<Value>,
    },

    /// Returns from the function with the given values as the return values.
    Return { values: Vec<Value> },

    /// Conditionally selects one of the two source registers based on the condition and stores the selected value in the destination register.
    Select {
        cond: Value,
        then_val: Value,
        else_val: Value,
        dest: Value,
    },

    /// Applies a signed integer binary operation to the two source registers and stores the result in the destination register.
    IBinOp {
        bin_op: IntBinOp,
        lhs: Value,
        rhs: Value,
        dest: Value,
    },

    /// Applies a floating-point binary operation to the two source registers and stores the result in the destination register.
    FBinOp {
        bin_op: FloatBinOp,
        lhs: Value,
        rhs: Value,
        dest: Value,
    },

    /// Applies a signed integer unary operation to the source register and stores the result in the destination register.
    IUnaryOp {
        unary_op: IntUnaryOp,
        operand: Value,
        dest: Value,
    },

    /// Applies a floating-point unary operation to the source register and stores the result in the destination register.
    FUnaryOp {
        unary_op: FloatUnaryOp,
        operand: Value,
        dest: Value,
    },
}
