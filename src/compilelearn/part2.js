function parseExpression(chars) {
    return parseMultiplicative(chars);
}

function parseMultiplicative(chars) {
    let left = parseNumber(chars);

    console.log(chars)
    while (chars.length > 0) {
        let ch = chars[0]; // 取第一个字符
        if (ch === '*') {
            chars.shift();  // 移除 '*' 并前进到下一个字符
            let right = parseNumber(chars);
            left = {
                type: 'Mul',  // 构建乘法表达式
                left: left,
                right: right
            };
        } else {
            break;
        }
    }

    return left;
}

function parseNumber(chars) {
    let numStr = '';
    while (chars.length > 0 && isDigit(chars[0])) {
        numStr += chars.shift(); // 逐个字符消耗
    }
    return {
        type: 'Num',
        value: parseInt(numStr)  // 将字符串转为整数
    };
}

function isDigit(char) {
    return /\d/.test(char);  // 检查是否为数字字符
}

// 测试
let input = '2*3*4'.split('');  // 将字符串拆分为字符数组
let expr = parseExpression(input);

console.log(JSON.stringify(expr, null, 2));
