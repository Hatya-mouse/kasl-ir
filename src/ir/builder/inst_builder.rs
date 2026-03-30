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

use crate::ir::{
    Block, Const, FloatBinOp, FloatUnaryOp, IRBuilder, IRType, Inst, IntBinOp, IntUnaryOp, Offset,
    Value, Variable,
    inst::{FloatCmp, IntCmp},
};

/// A trait for building instructions in the IRBuilder.
pub trait InstBuilder {
    /// Allocates memory on the stack with the given size and the alignment, and returns the allocated pointer.
    fn alloc(&mut self, size: Offset, alignment: Offset) -> Value;

    /// Loads a value from the source pointer and returns the loaded value.
    fn load(&mut self, ty: IRType, src_ptr: Value, src_offset: Offset) -> Value;

    /// Stores a source value to the destination pointer.
    fn store(&mut self, ty: IRType, src: Value, dst_ptr: Value, dst_offset: Offset);

    /// Copies the value stored in the source pointer to the destination pointer.
    fn memcpy(
        &mut self,
        size: Value,
        src_ptr: Value,
        src_offset: Offset,
        dst_ptr: Value,
        dst_offset: Offset,
    );

    /// Declares a constant variable and returns the value.
    fn const_val(&mut self, const_val: Const) -> Value;

    /// Assigns the value to the variable.
    fn assign(&mut self, var: Variable, src: Value);

    /// Jumps to the target block with the given arguments.
    fn jump(&mut self, block: Block, args: &[Value]);

    /// Contidionally jumps to the then block if the condition is not zero, otherwise jumps to the else block.
    fn brif(
        &mut self,
        cond: Value,
        then_block: Block,
        then_args: &[Value],
        else_block: Block,
        else_args: &[Value],
    );

    /// Returns from the function with the given values as the return values.
    fn _return(&mut self, vals: &[Value]);

    /// Conditionally selects one of the two source registers based on the condition, and returns the selected value.
    fn select(&mut self, cond: Value, then_val: Value, else_val: Value) -> Value;

    /// Applies a signed integer binary operation to the two source registers and returns the result.
    fn ibop(&mut self, op: IntBinOp, lhs: Value, rhs: Value) -> Value;

    /// Applies a floating-point binary operation to the two source registers and returns the result.
    fn fbop(&mut self, op: FloatBinOp, lhs: Value, rhs: Value) -> Value;

    /// Applies a signed integer unary operation to the source register and returns the result.
    fn iuop(&mut self, op: IntUnaryOp, operand: Value) -> Value;

    /// Applies a floating-point unary operation to the source register and returns the result.
    fn fuop(&mut self, op: FloatUnaryOp, operand: Value) -> Value;

    /// Performs an integer comparison between the two source registers and returns the result in 8-bit integer value.
    fn icmp(&mut self, cmp: IntCmp, lhs: Value, rhs: Value) -> Value;

    /// Performs a floating-point comparison between the two source registers and returns the result in 8-bit integer value.
    fn fcmp(&mut self, cmp: FloatCmp, lhs: Value, rhs: Value) -> Value;
}

impl InstBuilder for IRBuilder {
    fn alloc(&mut self, size: Offset, alignment: Offset) -> Value {
        // Create a value with pointer type to store the allocated pointer
        let dst = self.create_val(IRType::Ptr);

        self.push_inst(Inst::Alloc {
            size,
            alignment,
            dst,
        });
        dst
    }

    fn load(&mut self, ty: IRType, src_ptr: Value, src_offset: Offset) -> Value {
        // Create a value to store the loaded value
        let dst = self.create_val(ty);

        self.push_inst(Inst::Load {
            ty,
            src_ptr,
            src_offset,
            dst,
        });
        dst
    }

    fn store(&mut self, ty: IRType, src: Value, dst_ptr: Value, dst_offset: Offset) {
        // Ensure that the type of the source value matches the specified type.
        assert!(
            self.is_val_type(src, ty),
            "Type of the store source value does not match the specified type {}",
            ty
        );

        self.push_inst(Inst::Store {
            ty,
            src,
            dst_ptr,
            dst_offset,
        });
    }

    fn memcpy(
        &mut self,
        size: Value,
        src_ptr: Value,
        src_offset: Offset,
        dst_ptr: Value,
        dst_offset: Offset,
    ) {
        // Ensure that type of the both source and destination pointers is pointer type.
        assert!(
            self.is_val_type(dst_ptr, IRType::Ptr),
            "Type of the memcpy destination pointer is not pointer type"
        );
        assert!(
            self.is_val_type(src_ptr, IRType::Ptr),
            "Type of the memcpy source pointer is not pointer type"
        );

        self.push_inst(Inst::Memcpy {
            size,
            src_ptr,
            src_offset,
            dst_ptr,
            dst_offset,
        });
    }

    fn const_val(&mut self, const_val: Const) -> Value {
        // Create a value to store the created contant value
        let dst = self.create_val(const_val.get_type());

        self.push_inst(Inst::Const {
            value: const_val,
            dst,
        });
        dst
    }

    fn assign(&mut self, var: Variable, src: Value) {
        // Check if the variable type matches the type of the source value
        let var_ty = self.get_var_type(var);
        assert!(
            self.is_val_type(src, var_ty),
            "Type of the assign source value does not match the variable type {}",
            var_ty
        );

        self.push_inst(Inst::Assign { var, src });
    }

    fn jump(&mut self, block: Block, args: &[Value]) {
        self.push_inst(Inst::Jump {
            block,
            args: args.to_vec(),
        });
    }

    fn brif(
        &mut self,
        cond: Value,
        then_block: Block,
        then_args: &[Value],
        else_block: Block,
        else_args: &[Value],
    ) {
        // Ensure that the type of the condition value is 8-bit integer type
        assert!(
            self.is_val_type(cond, IRType::I8),
            "Type of the brif condition value must be 8-bit integer type"
        );

        self.push_inst(Inst::Brif {
            cond,
            then_block,
            then_args: then_args.to_vec(),
            else_block,
            else_args: else_args.to_vec(),
        });
    }

    fn _return(&mut self, vals: &[Value]) {
        self.push_inst(Inst::Return {
            vals: vals.to_vec(),
        });
    }

    fn select(&mut self, cond: Value, then_val: Value, else_val: Value) -> Value {
        // Ensure that the type of the condition value is 8-bit integer type
        assert!(
            self.is_val_type(cond, IRType::I8),
            "Type of the select condition value must be 8-bit integer type"
        );
        // Ensure that the type of the then value and else value are the same
        let then_ty = self.get_val_type(then_val);
        let else_ty = self.get_val_type(else_val);
        assert_eq!(
            then_ty, else_ty,
            "Type of the select then value {} does not match the type of the else value {}",
            then_ty, else_ty
        );

        // Create a value to store the selected value
        let dst = self.create_val(then_ty);

        self.push_inst(Inst::Select {
            cond,
            then_val,
            else_val,
            dst,
        });
        dst
    }

    fn ibop(&mut self, op: IntBinOp, lhs: Value, rhs: Value) -> Value {
        // Ensure that the type of the lhs value and rhs value are the same type
        let lhs_ty = self.get_val_type(lhs);
        let rhs_ty = self.get_val_type(rhs);
        assert_eq!(
            lhs_ty, rhs_ty,
            "Type of the binary op lhs {} does not match the type of the rhs {}",
            lhs_ty, rhs_ty
        );
        // Ensure that the type of the lhs value and rhs value are integer type
        assert!(
            lhs_ty.is_int(),
            "Type of the lhs is expected to be integer but got {}",
            lhs_ty
        );
        assert!(
            rhs_ty.is_int(),
            "Type of the rhs is expected to be integer but got {}",
            rhs_ty
        );

        // Create a value to store the result
        let dst = self.create_val(lhs_ty);

        self.push_inst(Inst::IBinOp { op, lhs, rhs, dst });
        dst
    }

    fn fbop(&mut self, op: FloatBinOp, lhs: Value, rhs: Value) -> Value {
        // Ensure that the type of the lhs value and rhs value are the same type
        let lhs_ty = self.get_val_type(lhs);
        let rhs_ty = self.get_val_type(rhs);
        assert_eq!(
            lhs_ty, rhs_ty,
            "Type of the binary op lhs {} does not match the type of the rhs {}",
            lhs_ty, rhs_ty
        );
        // Ensure that the type of the lhs value and rhs value are float type
        assert!(
            lhs_ty.is_float(),
            "Type of the lhs is expected to be float but got {}",
            lhs_ty
        );
        assert!(
            rhs_ty.is_float(),
            "Type of the rhs is expected to be float but got {}",
            rhs_ty
        );

        // Create a value to store the result
        let dst = self.create_val(lhs_ty);

        self.push_inst(Inst::FBinOp { op, lhs, rhs, dst });
        dst
    }

    fn iuop(&mut self, op: IntUnaryOp, operand: Value) -> Value {
        // Ensure that the type of the operand value is integer type
        let operand_ty = self.get_val_type(operand);
        assert!(
            operand_ty.is_int(),
            "Type of the unary op operand is expected to be integer but got {}",
            operand_ty
        );

        // Create a value to store the result
        let dst = self.create_val(operand_ty);

        self.push_inst(Inst::IUnaryOp { op, operand, dst });
        dst
    }

    fn fuop(&mut self, op: FloatUnaryOp, operand: Value) -> Value {
        // Ensure that the type of the operand value is float type
        let operand_ty = self.get_val_type(operand);
        assert!(
            operand_ty.is_float(),
            "Type of the unary op operand is expected to be float but got {}",
            operand_ty
        );

        // Create a value to store the result
        let dst = self.create_val(operand_ty);

        self.push_inst(Inst::FUnaryOp { op, operand, dst });
        dst
    }

    fn icmp(&mut self, cmp: IntCmp, lhs: Value, rhs: Value) -> Value {
        // Ensure that the type of the lhs value and rhs value are the same type
        let lhs_ty = self.get_val_type(lhs);
        let rhs_ty = self.get_val_type(rhs);
        assert_eq!(
            lhs_ty, rhs_ty,
            "Type of the int cmp lhs {} does not match the type of the rhs {}",
            lhs_ty, rhs_ty
        );
        // Ensure that the type of the lhs value and rhs value are integer type
        assert!(
            lhs_ty.is_int(),
            "Type of the lhs is expected to be integer but got {}",
            lhs_ty
        );
        assert!(
            rhs_ty.is_int(),
            "Type of the rhs is expected to be integer but got {}",
            rhs_ty
        );

        // Create a value to store the result
        let dst = self.create_val(IRType::I8);

        self.push_inst(Inst::ICmp { cmp, lhs, rhs, dst });
        dst
    }

    fn fcmp(&mut self, cmp: FloatCmp, lhs: Value, rhs: Value) -> Value {
        // Ensure that the type of the lhs value and rhs value are the same type
        let lhs_ty = self.get_val_type(lhs);
        let rhs_ty = self.get_val_type(rhs);
        assert_eq!(
            lhs_ty, rhs_ty,
            "Type of the float cmp lhs {} does not match the type of the rhs {}",
            lhs_ty, rhs_ty
        );
        // Ensure that the type of the lhs value and rhs value are float type
        assert!(
            lhs_ty.is_float(),
            "Type of the lhs is expected to be float but got {}",
            lhs_ty
        );
        assert!(
            rhs_ty.is_float(),
            "Type of the rhs is expected to be float but got {}",
            rhs_ty
        );

        // Create a value to store the result
        let dst = self.create_val(IRType::I8);

        self.push_inst(Inst::FCmp { cmp, lhs, rhs, dst });
        dst
    }
}
