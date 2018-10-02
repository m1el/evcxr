// Copyright 2018 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate failure;
extern crate json;
extern crate libc;
extern crate proc_macro2;
extern crate regex;
extern crate syn;
extern crate tempfile;
#[macro_use]
extern crate lazy_static;
extern crate backtrace;
extern crate libloading;
extern crate rand;
#[cfg(unix)]
#[macro_use]
extern crate sig;
#[cfg(windows)]
extern crate winapi;

#[macro_use]
mod errors;
mod child_process;
mod code_block;
mod command_context;
mod eval_context;
#[allow(dead_code, private_no_mangle_fns)]
mod evcxr_internal_runtime;
mod idents;
mod item;
mod module;
mod runtime;
mod statement_splitter;

pub use command_context::CommandContext;
pub use errors::{CompilationError, Error};
pub use eval_context::{EvalContext, EvalContextOutputs, EvalOutputs};
pub use runtime::runtime_hook;
