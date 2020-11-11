#[spandoc::spandoc]
fn _dont_panic_return_value() -> i32 {
    42
}

#[spandoc::spandoc]
fn _dont_panic_arg_return_value(_arg: i32) -> i32 {
    42
}

#[spandoc::spandoc]
fn panic_return_value_with_match() -> i32 {
    match 42 {
        _ => {},
    };

    42
}

#[spandoc::spandoc]
fn _panic_no_arg_or_return() {}

#[spandoc::spandoc]
fn _panic_arg_but_no_return(_arg: i32) {}

#[spandoc::spandoc]
fn main() {
}
