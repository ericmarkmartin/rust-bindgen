/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


extern "C" {
    #[link_name = "_Z3fooPKcz"]
    pub fn foo(fmt: *const ::std::os::raw::c_schar, ...);
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct Bar {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(::std::mem::size_of::<Bar>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Bar ) ));
    assert_eq! (::std::mem::align_of::<Bar>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Bar ) ));
}
extern "C" {
    #[link_name = "_ZN3Bar3fooEPKcz"]
    pub fn Bar_foo(this: *mut Bar, fmt: *const ::std::os::raw::c_schar, ...);
}
impl Clone for Bar {
    fn clone(&self) -> Self { *self }
}