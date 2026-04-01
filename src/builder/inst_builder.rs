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

use crate::{
    Block, Const, FloatBinOp, FloatUnaryOp, IRBuilder, IRType, Inst, IntBinOp, IntUnaryOp, Offset,
    Value, Variable,
    inst::{FloatCmp, IntCmp},
};

/// A trait for building instructions in the IRBuilder.
pub trait InstBuilder {
    /// Allocates memory on the stack with the given size and the alignment, and returns the allocated pointer.
    fn alloc(&mut self, size: u32, align: u32) -> Value;

    /// Loads a value from the source pointer and returns the loaded value.
    fn load(&mut self, ty: IRType, src_ptr: Value, src_offset: Offset) -> Value;

    /// Stores a source value to the destination pointer.
    fn store(&mut self, src: Value, dst_ptr: Value, dst_offset: Offset);

    /// Copies the value stored in the source pointer to the destination pointer.
    fn memcpy(&mut self, size: u32, src_ptr: Value, dst_ptr: Value);

    /// Fills the destination pointer with the given value.
    fn memset(&mut self, size: u32, value: u8, dst_ptr: Value);

    /// Declares a constant variable and returns the value.
    fn const_val(&mut self, const_val: Const) -> Value;

    /// Assigns the value to the variable.
    fn assign(&mut self, var: Variable, src: Value);

    /// Loads the value in the variable.
    fn load_var(&mut self, var: Variable) -> Value;

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

    /// Resizes the integer value to the specified type and returns the resized value.
    fn iresize(&mut self, src: Value, dst_ty: IRType) -> Value;

    /// Resizes the floating-point value to the specified type and returns the resized value.
    fn fresize(&mut self, src: Value, dst_ty: IRType) -> Value;

    /// Converts the integer value to the specified floating-point type.
    fn itof(&mut self, src: Value) -> Value;

    /// Converts the floating-point value to the specified integer type.
    fn ftoi(&mut self, src: Value) -> Value;

    /// Adds the offset to the pointer and returns the resulting pointer.
    fn ptr_add(&mut self, ptr: Value, offset: Offset) -> Value;

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

    /// Performs an integer comparison with an immediate value, and returnrs the result in 8-bit integer value.
    fn icmp_imm(&mut self, cmp: IntCmp, lhs: Value, rhs: i64) -> Value;

    /// Performs an floating-point comparison with an immediate value, and returnrs the result in 8-bit integer value.
    fn fcmp_imm(&mut self, cmp: FloatCmp, lhs: Value, rhs: f64) -> Value;
}

impl InstBuilder for IRBuilder {
    fn alloc(&mut self, size: u32, align: u32) -> Value {
        // Create a value with pointer type to store the allocated pointer
        let dst = self.create_val(IRType::Ptr);

        self.push_inst(Inst::Alloc { size, align, dst });
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

    fn store(&mut self, src: Value, dst_ptr: Value, dst_offset: Offset) {
        self.push_inst(Inst::Store {
            src,
            dst_ptr,
            dst_offset,
        });
    }

    fn memcpy(&mut self, size: u32, src_ptr: Value, dst_ptr: Value) {
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
            dst_ptr,
        });
    }

    fn memset(&mut self, size: u32, value: u8, dst_ptr: Value) {
        self.push_inst(Inst::Memset {
            size,
            value,
            dst_ptr,
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

    fn load_var(&mut self, var: Variable) -> Value {
        let var_ty = self.get_var_type(var);
        let dst = self.create_val(var_ty);
        self.push_inst(Inst::LoadVar { var, dst });
        dst
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

    fn iresize(&mut self, src: Value, dst_ty: IRType) -> Value {
        // Check if the source value is integer type and the destination type is also integer type
        let src_ty = self.get_val_type(src);
        assert!(
            src_ty.is_int(),
            "Source value of iresize must be integer type"
        );
        assert!(
            dst_ty.is_int(),
            "Destination type of iresize must be integer type"
        );

        // Create a value to store the resized value
        let dst = self.create_val(dst_ty);

        self.push_inst(Inst::IResize { src, dst_ty, dst });
        dst
    }

    fn fresize(&mut self, src: Value, dst_ty: IRType) -> Value {
        // Check if the source value is integer type and the destination type is also integer type
        let src_ty = self.get_val_type(src);
        assert!(
            src_ty.is_float(),
            "Source value of fresize must be integer type"
        );
        assert!(
            dst_ty.is_float(),
            "Destination type of fresize must be integer type"
        );

        // Create a value to store the resized value
        let dst = self.create_val(dst_ty);

        self.push_inst(Inst::FResize { src, dst_ty, dst });
        dst
    }

    fn itof(&mut self, src: Value) -> Value {
        // Check if the source value is integer type
        let src_ty = self.get_val_type(src);

        // Get the destination type
        let dst_ty = match src_ty {
            IRType::I8 | IRType::I16 => {
                panic!("Integer type smaller than 32-bit is not supported in itof")
            }
            IRType::I32 => IRType::F32,
            IRType::I64 => IRType::F64,
            _ => panic!("Source value of int_to_float must be integer type"),
        };
        // Create a value to store the resized value
        let dst = self.create_val(dst_ty);

        self.push_inst(Inst::IToF { src, dst_ty, dst });
        dst
    }

    fn ftoi(&mut self, src: Value) -> Value {
        // Check if the source value is float type
        let src_ty = self.get_val_type(src);

        // Get the destination type
        let dst_ty = match src_ty {
            IRType::F32 => IRType::I32,
            IRType::F64 => IRType::I64,
            _ => panic!("Source value of float_to_int must be float type"),
        };
        // Create a value to store the resized value
        let dst = self.create_val(dst_ty);

        self.push_inst(Inst::FToI { src, dst_ty, dst });
        dst
    }

    fn ptr_add(&mut self, ptr: Value, offset: Offset) -> Value {
        let dst = self.create_val(IRType::Ptr);
        self.push_inst(Inst::PtrAdd { ptr, offset, dst });
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

    fn icmp_imm(&mut self, cmp: IntCmp, lhs: Value, rhs: i64) -> Value {
        // Ensure that the type of the lhs value is integer type
        let lhs_ty = self.get_val_type(lhs);
        assert!(
            lhs_ty.is_int(),
            "Type of the lhs is expected to be integer but got {}",
            lhs_ty
        );

        // Create a value to store the result
        let dst = self.create_val(IRType::I8);

        self.push_inst(Inst::ICmpImm { cmp, lhs, rhs, dst });
        dst
    }

    fn fcmp_imm(&mut self, cmp: FloatCmp, lhs: Value, rhs: f64) -> Value {
        // Ensure that the type of the lhs value is float type
        let lhs_ty = self.get_val_type(lhs);
        assert!(
            lhs_ty.is_float(),
            "Type of the lhs is expected to be float but got {}",
            lhs_ty
        );

        // Create a value to store the result
        let dst = self.create_val(IRType::I8);

        self.push_inst(Inst::FCmpImm { cmp, lhs, rhs, dst });
        dst
    }
}
