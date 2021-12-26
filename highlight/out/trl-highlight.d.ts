declare enum HighlightTemplate {
    HTMLDocument = 0,
    CodeBlock = 1,
    None = 2
}
export declare class Highlighter {
    private src;
    private current;
    constructor(src: string);
    highlight(template?: HighlightTemplate): string;
}
export {};
