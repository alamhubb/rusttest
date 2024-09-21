use std::fs;
use std::env;





fn main() -> std::io::Result<()> {
    // 打印当前工作目录
    let current_dir = env::current_dir()?;
    println!("当前工作目录: {:?}", current_dir);

    // 创建一个空的 Vec，使用泛型 T 指定元素类型
    let mut string_vec: Vec<String> = Vec::new();

    // 添加字符串到 Vec 中
    string_vec.push(String::from("const"));
    string_vec.push(String::from("="));


    // 读取文件
    let content = fs::read_to_string("./src/test.js")?;

    // 打印文件内容
    println!("文件内容:\n{}", content);

    Ok(())
}