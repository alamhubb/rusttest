function lexer(input) {
    const tokens = [];
    let current = 0;

    while (current < input.length) {
        let char = input[current];

        // 跳过空白字符
        if (/\s/.test(char)) {
            current++;
            continue;
        }

        // 匹配字母和数字（单词）
        if (/\w/.test(char)) {
            let value = '';

            // 继续读取单词字符
            while (current < input.length && /\w/.test(input[current])) {
                value += input[current];
                current++;
            }

            tokens.push({ type: 'WORD', value });
            continue;
        }

        // 处理其他字符
        tokens.push({ type: 'UNKNOWN', value: char });
        current++;
    }

    return tokens;
}

// 测试代码
const input = `function testa() {
    function dudu(test){
        console.log(test)
    }
    const abc = '1 23'
    const dfds = 'fsadf'
    console.log(abc)
}`;

const tokens = lexer(input);
console.log(JSON.stringify(tokens, null, 2));