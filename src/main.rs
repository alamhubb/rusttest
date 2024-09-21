use std::str::Chars;

#[derive(Debug)]
enum Expr {
    Num(i32),                  // 数字表达式
    Mul(Box<Expr>, Box<Expr>),  // 乘法表达式
}

// 解析整个表达式，从乘法表达式开始
fn parse_expression(chars: &mut Chars) -> Expr {
    parse_multiplicative(chars)
}

// 解析乘法表达式 (multiplicativeExpression)
fn parse_multiplicative(chars: &mut Chars) -> Expr {
    let mut left = parse_number(chars);  // 首先解析一个数字 (IntLiteral)

    while let Some(ch) = chars.clone().next() {
        if ch == '*' {
            chars.next();  // 跳过 '*'
            let right = parse_number(chars);  // 解析右边的数字 (IntLiteral)
            left = Expr::Mul(Box::new(left), Box::new(right));  // 构建乘法表达式
        } else {
            break;
        }
    }

    left
}

// 解析整数字面量 (IntLiteral)
fn parse_number(chars: &mut Chars) -> Expr {
    let mut num = 0;
    while let Some(ch) = chars.clone().next() {
        if ch.is_digit(10) {
            num = num * 10 + ch.to_digit(10).unwrap() as i32;  // 将字符转为数字
            chars.next();  // 消耗当前字符
        } else {
            break;
        }
    }

    Expr::Num(num)  // 返回一个数字表达式
}

fn main() {
    let input = "2*3*4";  // 示例表达式
    let mut chars = input.chars();
    let expr = parse_expression(&mut chars);  // 解析输入的表达式

    println!("{:?}", expr);  // 输出解析结果
}
