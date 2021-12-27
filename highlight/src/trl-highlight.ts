function isNumeric(a: string): boolean {
    if (a.match(/[0-9]/)) return true;
    else return false;
}

function char_in_str(c: string, l: string): boolean {
    for (let i = 0; i < l.length; i++) if (c == l[i]) return true;
    return false;
}

enum HighlightTemplate {
    HTMLDocument,
    CodeBlock,
    None,
}

class Highlighter {
    private src: string;
    private current: number;

    constructor(src: string) {
        this.src = src;
        this.current = 0;
    }

    public highlight(
        template: HighlightTemplate = HighlightTemplate.None
    ): string {
        let result = "";

        switch (template) {
            case HighlightTemplate.CodeBlock:
                result += `\
<div class="codeblock">
    <div class="codeblock-header">
        <div>tr-lang</div>
        <div>Copy</div>
    </div>
    <pre class="code">`;
                break;
            case HighlightTemplate.HTMLDocument:
                result += `\
<doctype! html>
<html>
    <head>
        <title> tr-lang Code Snippet </title>
    </head>
    <body>
        <div class="codeblock">
            <div class="codeblock-header">
                <div>tr-lang</div>
                <div>Copy</div>
            </div>
            <pre class="code">`;
                break;
            case HighlightTemplate.None:
                break;
        }

        while (this.current < this.src.length) {
            let c = this.src[this.current];

            switch (true) {
                case c == '#':
                    result += '<span class="comment comment:line">';
                    while (this.current < this.src.length) {
                        result += this.src[this.current];
                        this.current++;
                        if (this.src[this.current] == '\n') break;
                    }
                    result += '</span>';
                    break;

                case c == '(':
                    result += '<span class="paren paren:left">(</span>';
                    this.current++;
                    break;

                case c == ')':
                    result += '<span class="paren paren:right">)</span>';
                    this.current++;
                    break;

                case c == ',':
                    result += '<span class="comma">,</span>';
                    this.current++;
                    break;

                case c == "'" || c == '"':
                    result += '<span class="string string:' + (c == '"'
                            ? 'double-quoted'
                            : 'single-quoted') + '">"';
                    this.current++;
                    while (this.src[this.current] != c) {
                        if (this.src[this.current] != '\\') {
                            result += this.src[this.current];
                            this.current++;
                        } else {
                            this.current++;
                            switch (this.src[this.current]) {
                                case 't':
                                case 'n':
                                case 'r':
                                case '"':
                                case "'":
                                case '\\':
                                case '\n':
                                case '\t':
                                    result += '<span class="string string:escape">\\' +
                                        this.src[this.current] +
                                        '</span>';
                                    break;

                                default:
                                    result += '\\' + this.src[this.current];
                                    break;
                            }
                            this.current++;
                        }
                    }
                    this.current++;
                    result += '"</span>';
                    break;

                case isNumeric(c): {
                    let dot_used = false;;
                    let buf = '';

                    while (
                        this.src.length > this.current &&
                        (isNumeric(this.src[this.current]) ||
                            this.src[this.current] == '.')
                    ) {
                        if (this.src[this.current] != '.') {
                            buf += this.src[this.current];
                        } else {
                            if (dot_used) {
                                buf += '<span class="error">.</span>';
                            } else {
                                dot_used = true;
                                buf += '.';
                            }
                        }
                        this.current++;
                    }

                    result += '<span class="number number:'+ (dot_used ? 'float' : 'int' )+'">' + buf + "</span>";

                    break;
                }

                case char_in_str(c, '\n '):
                    result += c;
                    this.current++;
                    break;

                case c == '+':
                    this.current++;
                    if (this.src.length > this.current) {
                        if (this.src[this.current] == '+') {
                            this.current++;
                            result += '<span class="operator operator:plusplus">++</span>';
                        } else {
                            result += '<span class="operator operator:plus">+</span>';
                        }
                    }

                case c == '-':
                    this.current++;
                    if (this.src.length > this.current) {
                        switch (this.src[this.current]) {
                            case "-":
                                this.current++;
                                result += '<span class="operator operator:minusminus">--</span>';
                                break;

                            case ">":
                                this.current++;
                                result += '<span class="operator operator:putinto">-></span>';
                                break;

                            case "*":
                                result += '<span class="comment comment:block">-*';
                                while (true) {
                                    this.current++;
                                    if (this.current > this.src.length) result += '<span class="error"> unterminated comment </span>';
                                    if (this.src[this.current] == '*') {
                                        this.current++;
                                        if (this.current < this.src.length) {
                                            if (this.src[this.current] == '-') {
                                                this.current++;
                                                result += '*-';
                                                break;
                                            }
                                        }
                                        else result += '<span class="error"> unterminated comment </span>';
                                    } else {
                                        result += this.src[this.current];
                                    }
                                }
                                result += '</span>'
                                break;

                            default:
                                result += '<span class="operator operator:minus">-</span>';
                                break;
                        }
                    }

                default:
                    let buf = "";

                    while (this.src.length > this.current
                        && !char_in_str(this.src[this.current], "\t\r \n\"':?=<>!/%*@,"))
                    {
                        buf += this.src[this.current];
                        this.current++;
                    }

                    switch (buf) {
                        case "at":
                            result += '<span class="keyword keyword:at">' + buf + '</span>';
                            break;

                        case "ver":
                            result += '<span class="keyword keyword:ver">' + buf + '</span>';
                            break;

                        case "de":
                            result += '<span class="keyword keyword:de">' + buf + '</span>';
                            break;

                        case "ise":
                            result += '<span class="keyword keyword:ise">' + buf + '</span>';
                            break;

                        case "son":
                            result += '<span class="keyword keyword:son">' + buf + '</span>';
                            break;

                        case "iken":
                            result += '<span class="keyword keyword:iken">' + buf + '</span>';
                            break;

                        case "yoksa":
                            result += '<span class="keyword keyword:yoksa">' + buf + '</span>';
                            break;

                        case "doğru":
                            result += '<span class="boolean boolean:doğru">' + buf + '</span>';
                            break;

                        case "yanlış":
                            result += '<span class="boolean boolean:yanlış">' + buf + '</span>';
                            break;

                        case "kpy":
                            result += '<span class="keyword keyword:kpy">' + buf + '</span>';
                            break;

                        case "tks":
                            result += '<span class="keyword keyword:tks">' + buf + '</span>';
                            break;

                        case "üst":
                            result += '<span class="keyword keyword:üst">' + buf + '</span>';
                            break;

                        case "veya":
                            result += '<span class="keyword keyword:veya">' + buf + '</span>';
                            break;

                        case "ve":
                            result += '<span class="keyword keyword:ve">' + buf + '</span>';
                            break;

                        case "dön":
                            result += '<span class="keyword keyword:dön">' + buf + '</span>';
                            break;

                        case "girdi":
                            result += '<span class="keyword keyword:girdi">' + buf + '</span>';
                            break;

                        case "işlev":
                            result += '<span class="keyword keyword:işlev">' + buf + '</span>';
                            break;

                        case "yükle":
                            result += '<span class="keyword keyword:yükle">' + buf + '</span>';
                            break;

                        default:
                            result += '<span class="identifier">' + buf + '</span>';
                            break;
                    }

                    break;
            }
        }

        switch (template) {
            case HighlightTemplate.CodeBlock:
                result += `</pre>
</div>`;
                break;
            case HighlightTemplate.HTMLDocument:
                result += `</pre>
        </div>
    </body>
</html>`;
                break;
            case HighlightTemplate.None:
                break;
        }

        return result;
    }
}
