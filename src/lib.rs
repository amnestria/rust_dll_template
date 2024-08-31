#![no_main]

use windows::Win32::{
    Foundation::{FreeLibrary, HMODULE},
    System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
};

#[no_mangle]
extern "system" fn DllMain(module: HMODULE, reason: u32, _: isize) -> bool {
    match reason {
        DLL_PROCESS_ATTACH => prologue(),
        DLL_PROCESS_DETACH => epilogue(module),
        _ => Ok(()),
    }
    .is_ok()
}

pub fn prologue() -> DllResult<()> {
    Ok(())
}

pub fn epilogue(module: HMODULE) -> DllResult<()> {
    unsafe { FreeLibrary(module).map_err(|_| DllError::Library)? };
    Ok(())
}

//*
//* ../error_handling
//*
#[derive(Debug)]
pub enum DllError {
    Library,
}

pub type DllResult<T> = core::result::Result<T, DllError>;
