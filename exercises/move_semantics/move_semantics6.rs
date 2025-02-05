// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    // 传递引用给 get_char 函数，避免所有权转移
    get_char(&data);

    // 传递数据本身给 string_uppercase 函数，让其获取所有权
    string_uppercase(data);
}

// 不获取所有权，接收引用
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 获取所有权，接收 String 类型
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    println!("{}", data);
}