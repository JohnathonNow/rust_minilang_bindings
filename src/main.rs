#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::ffi::c_void;
use std::ffi::CStr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
fn main() {
    unsafe {
    let NULL = 0 as *mut c_void;
    let Globals = stringmap_new();
   ml_init();
   ml_types_init(Globals);
   let Parameters = ["Args".as_ptr(), 0 as *const u8];
   let Args = ml_list();
   let FileName = "example.ml";
   let a = &mut MLMain;
   let State = ml_call_state_new(a as *mut ml_state_t, 1);
   *(*State).Args.as_mut_ptr() = Args;
   ml_load_file(State as *mut ml_state_t, Some(std::mem::transmute::<*const (), unsafe extern "C" fn(*mut c_void, *const i8) -> *mut ml_value_t>(ml_global_get as *const ())), Globals as *mut c_void, FileName.as_ptr() as *const i8, NULL as *mut *const i8);
    }
}
