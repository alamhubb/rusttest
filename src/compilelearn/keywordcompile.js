//声明js中涉及到的关键词
const keywords = ['function', 'const', '(', ')', '{', '}', '=', '.']


//声明标识符生成规则
const identifierReg = /^[a-zA-Z$_][a-zA-Z$_\d]*/
//声明字符串字面量生成规则
const stringLiteralReg = /^('.+?')/
const whitespaceReg = /^\s+/

let parserCode = `function testa() {
    function dudu(test){
        console.log(test)
    }
    const abc = '1 23'
    const dfds = 'fsadf'
    console.log(abc)
}
`
//去除空白字符，生成简短的语句，主要是去除空白字符，并且保持分割
const tokens = []
//遍历短语句
//根据每个短语句生成token
while (parserCode) {
    const identifierResult = parserCode.match(identifierReg)
    if (identifierResult && identifierResult[0]) {
        const id = identifierResult[0]
        //然后拆分token，以什么开头，则为什么token，加入到token数组，并从code中删除
        if (isKeyword(id)) {
            tokens.push({type: id, token: id})
            parserCode = parserCode.substring(id.length)
        } else {
            tokens.push({type: 'identifier', token: id})
        }
        parserCode = parserCode.substring(id.length)
        continue
    }
    const id = isKeyword(parserCode)
    if (id) {
        tokens.push({type: id, token: id})
        parserCode = parserCode.substring(id.length)
        continue
    }
    const stringLiteralResult = parserCode.match(stringLiteralReg)
    if (stringLiteralResult && stringLiteralResult[0]) {
        tokens.push({type: 'stringLiteral', token: stringLiteralResult[0]})
        parserCode = parserCode.substring(stringLiteralResult[0].length)
        continue
    }
    const whitespaceResult = parserCode.match(whitespaceReg)
    if (whitespaceResult && whitespaceResult[0]) {
        // console.log(`'${ whitespaceResult[0] }'`)
        // console.log(whitespaceResult[0].length)
        // console.log(parserCode)
        parserCode = parserCode.substring(whitespaceResult[0].length)
        continue
    }
    console.log('错误：' + parserCode)
    break
}
//打印tokens结果
console.log(tokens)

function isKeyword(code) {
    for (const keyword of keywords) {
        if (code.startsWith(keyword)) {
            return keyword
        }
    }
}
