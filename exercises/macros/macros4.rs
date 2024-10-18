// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.



macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();      // Calls the first arm of the macro
    my_macro!(7777);  // Calls the second arm of the macro
}
