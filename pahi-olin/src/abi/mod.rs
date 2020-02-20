/// The Olin ABI implemented for pa'i. This mostly contains rigging and other
/// internal implementation details.

/// Logging support
pub mod log;

/// Simple entropy
pub mod random;

/// Runtime interop
pub mod runtime;

/// Time
pub mod time;

use crate::Process;
use wasmer_runtime::{Array, Ctx, WasmPtr};

pub fn io_get_stderr(_: &mut Ctx) -> u32 {
    0
}

pub fn resource_write(
    ctx: &mut Ctx,
    _fd: u32,
    _base: WasmPtr<u8, Array>,
    len: u32,
) -> Result<u32, wasmer_runtime::error::Error> {
    let (_, env) = Process::get_memory_and_environment(ctx, 0);
    env.log_call("resource_write".to_string());
    Ok(len)
}
