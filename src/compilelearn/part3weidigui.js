function parseMultiplicativeTail(chars, left) {
    if (chars.length > 0 && chars[0] === '*') {
        chars.shift(); // 消耗 '*'
        let right = parseNumber(chars); // 解析下一个数字
        return parseMultiplicativeTail(chars, {
            type: 'Mul',
            left: left,
            right: right
        }); // 尾递归调用
    }
    return left; // 如果没有 '*', 返回结果
}

function parseNumber(chars) {
    let numStr = '';
    while (chars.length > 0 && isDigit(chars[0])) {
        numStr += chars.shift(); // 消耗数字字符
    }
    return {
        type: 'Num',
        value: parseInt(numStr, 10) // 将字符串转换为整数
    };
}

function isDigit(char) {
    return /\d/.test(char); // 检查是否为数字字符
}

// 测试
let input = '2*3*4'.split(''); // 输入的字符数组
let expr = parseMultiplicative(input, parseNumber(input));

console.log(JSON.stringify(expr, null, 2));
