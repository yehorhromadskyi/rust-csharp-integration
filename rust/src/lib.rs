use libc::{c_char};
use std::ffi::CString;
use std::ffi::CStr;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[no_mangle]
pub extern fn dealloc(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
            CString::from_raw(s)
    };
}

#[no_mangle]
pub extern fn send_bytes() -> *const u8 {
    let bts: &[u8] = &[99, 69];
    println!("{:?}", bts.len());
    bts.as_ptr()
}

#[repr(C)] 
#[derive(Serialize, Deserialize, Debug)]
pub struct MyStruct {
    pub id: i32,
    pub flag_true: bool,
    pub flag_false: bool,
    pub name: String,
    pub numbers: Vec<i32>
}

#[no_mangle]
pub extern fn get_struct() -> *mut c_char {
   let st = MyStruct { id: 7, flag_true: true, flag_false: false, name: String::from("Khan"), numbers: vec![1, 2, 3] };
   let serialized = serde_json::to_string(&st).unwrap();
   CString::new(serialized).unwrap().into_raw()
}

#[no_mangle]
pub extern fn set_struct(s: *const c_char) {
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap();
    let my_struct: MyStruct = serde_json::from_str(r_str).unwrap();
    println!("{:#?}", my_struct);
}

