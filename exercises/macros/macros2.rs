// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


// 先定义宏
macro_rules! my_macro {
    ($msg:expr) => {
        println!("{}", $msg);
    };
}

fn main() {
    // 然后调用宏
    my_macro!("Hello from the macro!");
}
