// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.


// 定义一个可以接收参数的宏
macro_rules! my_macro {
    ($message:expr) => {
        println!("{}", $message);
    };
}

fn main() {
    // 调用宏并传入一个字符串作为参数
    my_macro!("Check out my improved macro!");
}
