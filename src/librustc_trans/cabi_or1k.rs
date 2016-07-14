// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use llvm;
use llvm::{Integer, Pointer, Float, Double, Struct, Array, Attribute};
use abi::{FnType, ArgType};
use context::CrateContext;
use type_::Type;

use std::cmp;

fn align_up_to(off: usize, a: usize) -> usize {
    return (off + a - 1) / a * a;
}

fn align(off: usize, ty: Type) -> usize {
    let a = ty_align(ty);
    return align_up_to(off, a);
}

fn ty_align(ty: Type) -> usize {
    match ty.kind() {
        Integer => {
            unsafe {
                ((llvm::LLVMGetIntTypeWidth(ty.to_ref()) as usize) + 7) / 8
            }
        }
        Pointer => 4,
        Float => 4,
        Double => 8,
        Struct => {
          if ty.is_packed() {
            1
          } else {
            let str_tys = ty.field_types();
            str_tys.iter().fold(1, |a, t| cmp::max(a, ty_align(*t)))
          }
        }
        Array => {
            let elt = ty.element_type();
            ty_align(elt)
        }
        _ => bug!("ty_size: unhandled type")
    }
}

fn ty_size(ty: Type) -> usize {
    match ty.kind() {
        Integer => {
            unsafe {
                ((llvm::LLVMGetIntTypeWidth(ty.to_ref()) as usize) + 7) / 8
            }
        }
        Pointer => 4,
        Float => 4,
        Double => 8,
        Struct => {
            if ty.is_packed() {
                let str_tys = ty.field_types();
                str_tys.iter().fold(0, |s, t| s + ty_size(*t))
            } else {
                let str_tys = ty.field_types();
                let size = str_tys.iter().fold(0, |s, t| align(s, *t) + ty_size(*t));
                align(size, ty)
            }
        }
        Array => {
            let len = ty.array_length();
            let elt = ty.element_type();
            let eltsz = ty_size(elt);
            len * eltsz
        }
        _ => bug!("ty_size: unhandled type")
    }
}

fn classify_ret_ty(ccx: &CrateContext, ret: &mut ArgType) {
    if is_reg_ty(ret.ty) {
        ret.extend_integer_width_to(32);
    } else {
        ret.make_indirect(ccx);
    }
}

fn classify_arg_ty(ccx: &CrateContext, arg: &mut ArgType, offset: &mut usize) {
    let size = ty_size(arg.ty) * 8;
    let mut align = ty_align(arg.ty);

    align = cmp::min(cmp::max(align, 4), 8);
    *offset = align_up_to(*offset, align);
    *offset += align_up_to(size, align * 8) / 8;

    if is_reg_ty(arg.ty) {
        arg.extend_integer_width_to(32);
    } else {
        arg.make_indirect(ccx);
        arg.attrs.set(Attribute::ByVal);
    }
}

fn is_reg_ty(ty: Type) -> bool {
    return match ty.kind() {
        Integer
        | Pointer
        | Float
        | Double => true,
        _ => false
    };
}

pub fn compute_abi_info(ccx: &CrateContext, fty: &mut FnType) {
    if !fty.ret.is_ignore() {
        classify_ret_ty(ccx, &mut fty.ret);
    }

    let mut offset = if fty.ret.is_indirect() { 4 } else { 0 };
    for arg in &mut fty.args {
        if arg.is_ignore() { continue; }
        classify_arg_ty(ccx, arg, &mut offset);
    }
}
