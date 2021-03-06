use crate::{
    resource::Resource,
    scheme::{Gemini, Http, Https, Log, Null, Random, Zero},
    *,
};
use log::debug;
use std::cell::RefCell;
use std::io::{Read, Write};
use url::Url;
use wasmer_runtime::{Array, Ctx, WasmPtr};

pub fn open(ctx: &mut Ctx, ptr: WasmPtr<u8, Array>, len: u32) -> Result<i32, error::Error> {
    let (memory, env) = Process::get_memory_and_environment(ctx, 0);
    env.log_call("resource_open".to_string());
    let violations = RefCell::new(Vec::new());
    let raw_uri = ptr
        .get_utf8_string(memory, len)
        .ok_or(error::Error::InvalidArgument)?;
    let uri = Url::options()
        .syntax_violation_callback(Some(&|v| violations.borrow_mut().push(v)))
        .parse(&raw_uri);
    env.log_url(raw_uri.to_string());

    match uri {
        Ok(uri) => {
            let fd = env.get_fd();
            return match uri.scheme() {
                "gemini" => match Gemini::new(uri) {
                    Ok(res) => {
                        env.resources.insert(fd, Box::new(res));
                        return Ok(fd as i32);
                    }
                    Err(why) => Ok(why as i32),
                },
                "http" => match Http::new(uri) {
                    Ok(res) => {
                        env.resources.insert(fd, Box::new(res));
                        return Ok(fd as i32);
                    }
                    Err(why) => Ok(why as i32),
                },
                "https" => match Https::new(uri) {
                    Ok(res) => {
                        env.resources.insert(fd, Box::new(res));
                        return Ok(fd as i32);
                    }
                    Err(why) => Ok(why as i32),
                },
                "log" => {
                    env.resources.insert(fd, Box::new(Log::new(uri).unwrap()));
                    Ok(fd as i32)
                }
                "null" => {
                    env.resources.insert(fd, Box::new(Null::new(uri).unwrap()));
                    Ok(fd as i32)
                }
                "random" => {
                    env.resources
                        .insert(fd, Box::new(Random::new(uri).unwrap()));
                    Ok(fd as i32)
                }
                "zero" => {
                    env.resources.insert(fd, Box::new(Zero::new(uri).unwrap()));
                    Ok(fd as i32)
                }
                _ => Ok(error::Error::InvalidArgument as i32),
            };
        }
        Err(why) => {
            log::error!("URL parsing error {}: {:?}", &raw_uri, why);
            Ok(error::Error::InvalidArgument as i32)
        }
    }
}

pub fn close(ctx: &mut Ctx, fd: u32) -> Result<(), ()> {
    let (_, env) = Process::get_memory_and_environment(ctx, 0);
    env.log_call("resource_close".to_string());

    if !env.resources.contains_key(&fd) {
        return Err(());
    }

    let resources = &mut env.resources.as_mut();
    let res = &mut resources.get_mut(&fd);
    let res = &mut res.as_mut().expect("wanted mutable ref");

    res.close();
    resources.remove(&fd);

    Ok(())
}

pub fn read(
    ctx: &mut Ctx,
    fd: u32,
    ptr: WasmPtr<u8, Array>,
    len: u32,
) -> Result<i32, std::option::NoneError> {
    let (memory, env) = Process::get_memory_and_environment(ctx, 0);
    env.log_call("resource_read".to_string());

    if !env.resources.contains_key(&fd) {
        return Ok(error::Error::InvalidArgument as i32);
    }

    let res = &mut env.resources.as_mut().get_mut(&fd);
    let res = res.as_mut()?;
    let mut data = vec![0; len as usize];

    match res.read(&mut data) {
        Err(why) => {
            log::error!("File read error: {:?}", why);
            Ok(error::Error::Unknown as i32)
        }
        Ok(read_len) => {
            if read_len == 0 {
                return Ok(error::Error::EOF as i32);
            }

            unsafe {
                let memory_writer = ptr.deref_mut(memory, 0, len)?;
                for (i, b) in data.bytes().enumerate() {
                    memory_writer[i].set(b.unwrap());
                }
            }

            Ok(len as i32)
        }
    }
}

pub fn write(
    ctx: &mut Ctx,
    fd: u32,
    ptr: WasmPtr<u8, Array>,
    len: u32,
) -> Result<i32, std::option::NoneError> {
    let (memory, env) = Process::get_memory_and_environment(ctx, 0);
    env.log_call("resource_write".to_string());
    debug!("write: {:?} {} {}", ptr, len, fd);

    if !env.resources.contains_key(&fd) {
        return Ok(error::Error::InvalidArgument as i32);
    }

    let res = &mut env.resources.as_mut().get_mut(&fd);
    let res = &mut res.as_mut()?;
    let reader = ptr.deref(memory, 0, len)?;
    let mut data = vec![0; len as usize];

    for (i, b) in reader.iter().enumerate() {
        data[i] = b.get();
    }

    if let Err(why) = res.write(&data) {
        log::error!("File write error: {:?}", why);
        return Ok(error::Error::Unknown as i32);
    }

    Ok(len as i32)
}

pub fn flush(ctx: &mut Ctx, fd: u32) -> Result<i32, std::option::NoneError> {
    let (_, env) = Process::get_memory_and_environment(ctx, 0);
    env.log_call("resource_flush".to_string());

    if !env.resources.contains_key(&fd) {
        return Ok(error::Error::InvalidArgument as i32);
    }

    let res = &mut env.resources.as_mut().get_mut(&fd);
    let res = &mut res.as_mut()?;

    if let Err(why) = res.flush() {
        log::error!("File flush error: {:?}", why);
    }

    Ok(0)
}
