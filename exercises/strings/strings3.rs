// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    // 使用 trim 方法去除字符串两端的空白字符，再将结果转换为 String 类型
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // 方法一：使用 format! 宏
    format!("{} world!", input)

    // 方法二：使用 + 运算符
    // let mut result = input.to_string();
    // result.push_str(" world!");
    // result
}

fn replace_me(input: &str) -> String {
    // 使用 replace 方法将字符串中的 "cars" 替换为 "balloons"
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
