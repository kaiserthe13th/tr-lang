function isNumeric(a: string): boolean {
    if (a.match(/[0-9]/)) return true;
    else return false;
}

enum HighlightTemplate {
    HTMLDocument,
    CodeBlock,
    None,
}

export class Highlighter {
    private src: string;
    private current: number;

    constructor(src: string) {
        this.src = src;
        this.current = 0;
    }

    public highlight(
        template: HighlightTemplate = HighlightTemplate.None
    ): string {
        let result = '';

        switch (template) {
            case HighlightTemplate.CodeBlock:
                result += `\
<div class="codeblock">
    <div class="codeblock-header">
        <div>tr-lang</div>
        <div>Copy</div>
    </div>
    <pre class="code"></pre>
</div>`;
                break;
            case HighlightTemplate.HTMLDocument:
                result += '';
                break;
            case HighlightTemplate.None:
                break;
        }

        while (this.current < this.src.length) {
            let c = this.src[this.current];

            switch (true) {
                case c == '#':
                    result += '<span class="comment:line">';
                    while (this.current < this.src.length) {
                        result += this.src[this.current];
                        this.current++;
                    }
                    result += '</span>';
                    break;

                case c == '(':
                    result += '<span class="paren:left">(</span>';
                    this.current++;
                    break;

                case c == ')':
                    result += '<span class="paren:right">)</span>';
                    this.current++;
                    break;

                case c == ',':
                    result += '<span class="comma">,</span>';
                    this.current++;
                    break;

                case c == "'" || c == '"':
                    result +=
                        '<span class="string:' + c == '"'
                            ? 'double-quoted'
                            : 'single-quoted' + '">';
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
                                    result +=
                                        '<span class="string:escape">\\' +
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
                    break;

                case isNumeric(c): {
                    let dot_used = false;
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

                    break;
                }
                default:
                    break;
            }
        }

        return result;
    }
}
