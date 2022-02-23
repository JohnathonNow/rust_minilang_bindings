#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::ffi::c_void;
use std::ffi::CStr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn as_string(p: *mut ml_value_t) -> String {
    unsafe {
    CStr::from_ptr((*(p as (*mut ml_address_t))).Value).to_str().unwrap().into()
    }
}

extern "C" fn print(Data: *mut c_void, Count: i32, Args: *mut *mut ml_value_t) -> *mut ml_value_t {
   println!("OMG GOT CALLED!");
    unsafe {
	let StringMethod = ml_method("string\0".as_ptr() as *const i8);
	for I in 0..Count {
		let mut R = *(Args.add(I as usize));
		if ((*R).Type != MLStringT.as_mut_ptr()) {
			R = ml_simple_call(StringMethod, 1, R as *mut *mut ml_value_t);
			if ((*R).Type == MLErrorT.as_mut_ptr()) {return R as *mut ml_value_t};
			if ((*R).Type != MLStringT.as_mut_ptr()) {return ml_error("ResultError\0".as_ptr() as *const i8, "string method did not return string\0".as_ptr() as *const i8);}
		}
                println!("{}", CStr::from_ptr((*(R as (*mut ml_address_t))).Value).to_str().unwrap());
	}
	return MLNil.as_mut_ptr() as *mut ml_value_t;
    }
}

fn main() {
    unsafe {
    let NULL = 0 as *mut c_void;
    let Globals = stringmap_new();
   ml_init();
   //ml_types_init(Globals);
   stringmap_insert(Globals, "print\0".as_ptr() as *const i8, ml_cfunction(NULL, Some(std::mem::transmute::<*const (), unsafe extern "C" fn(*mut c_void, i32, *mut *mut ml_value_t) -> *mut ml_value_t>(print as *const ()))) as *mut c_void);
   let FileName = "example.ml\0";
   let State = ml_result_state_new(NULL as *mut ml_context_t);
   println!("{:?}", (*State).Value);
   ml_load_file(State as *mut ml_state_t, Some(std::mem::transmute::<*const (), unsafe extern "C" fn(*mut c_void, *const i8) -> *mut ml_value_t>(stringmap_search as *const ())), Globals as *mut c_void, FileName.as_ptr() as *const i8, NULL as *mut *const i8);
   println!("GOING NOW!");
   (*(*(*State).Value).Type).call.unwrap()(State as *mut ml_state_t, (*State).Value, 0, NULL as *mut *mut ml_value_t);
   println!("{:?}", (*State).Value);
    //println!("{}", CStr::from_ptr((*((State) as (*mut ml_address_t))).Value).to_str().unwrap());
    }
}
