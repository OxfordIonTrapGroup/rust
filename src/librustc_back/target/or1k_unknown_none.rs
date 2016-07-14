// Copyright 2012-2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use LinkerFlavor;
use target::{Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    Ok(Target {
        llvm_target: "or1k-unknown-none".to_string(),
        target_endian: "big".to_string(),
        target_pointer_width: "32".to_string(),
        target_os: "none".to_string(),
        target_env: "".to_string(),
        target_vendor: "unknown".to_string(),
        arch: "or1k".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        data_layout: "E-m:e-p:32:32-i8:8:8-i16:16:16-i64:32:32-f64:32:32-v64:32:32-v128:32:32-a0:0:32-n32".to_string(),
        options: TargetOptions {
            min_atomic_width: Some(32),
            max_atomic_width: Some(32),
            target_family: Some("none".to_string()),
            ..Default::default()
        }
    })
}
