// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.


fn main() {
    // 无需在 main 函数里预先创建 Vec
    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` 不再接收参数，而是在内部创建并返回 Vec<i32>
fn fill_vec() -> Vec<i32> {
    // 在函数内部创建一个空的 Vec<i32>
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    // 返回填充好元素的 Vec
    vec
}