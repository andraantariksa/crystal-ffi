#[no_mangle]
fn number_two() -> i32 {
    2
}

#[no_mangle]
fn str_string_hello() -> &'static str {
    "Hello Crystal! This is a &str\n"
}

// #[no_mangle]
// fn string_hello() -> String {
//     format!("Hello Crystal! This is a String\nFrom: Rust")
// }

#[no_mangle]
fn const_u8_ptr_string_hello() -> *const u8 {
    format!("Hello Crystal! This is a *const u8 string\n\0").as_ptr()
}
