/*
Language: tr-lang
Description: tr-lang is a programming language that aims to make syntax like Turkish.
Website: https://github.com/kaiserthe13th/tr-lang
Author: Kerem Göksu <superkerem13@gmail.com>
*/

export default function(hljs) {
    const IDENT = {
        match: /[^\t\r \n"':?=<>!\/%\*@,\d-][^\t\r \n"':?=<>!\/%\*@,]+/i,
        scope: 'variable'
    };

    const RESERVED_WORDS = [
        "at",
        "ver",
        "de",
        "ise",
        "son",
        "iken",
        "yoksa",
        "kpy",
        "tks",
        "üst",
        "veya",
        "ve",
        "dön",
        "girdi",
        "işlev",
        "yükle",
    ];

    const LINE_COMMENT = {
        scope: 'comment',
        begin: /#/,
        end: /$/,
    };

    const MULTI_CHAR_OPS = {
        scope: 'operator',
        match: /->|=\?|:\?|:\./,
    };

    const SINGLE_CHAR_OPS = {
        scope: 'operator',
        match: /[@\+\*%>=!<]/,
    };

    const COMMON_OPS = {
        className: 'operator',
        variants: [
            MULTI_CHAR_OPS,
            SINGLE_CHAR_OPS,
        ],
    };

    const BLOCK_COMMENT = {
        scope: 'comment',
        begin: /-\*/,
        end: /\*-/,
        relevance: 10,
    };

    const NUMBER = {
        className: 'number',
        begin: /\d+(\.\d*)?/,
    };

    const LITERALS = [
        'doğru',
        'yanlış'
    ]

    const KEYWORDS = {
        $pattern: IDENT.match,
        keyword: RESERVED_WORDS,
        literals: LITERALS,
    };

    const STR_ESC = {
        begin: /\\[tnr"'\\\n\t]/,
        scope: 'char.escape',
        relevance: 0,
    };

    const SINGLE_QUOTE_STR = {
        begin: "'",
        end: "'",
        contains: [
            STR_ESC
        ],
    };

    const DOUBLE_QUOTE_STR = {
        scope: 'string',
        begin: '"',
        end: '"',
        contains: [
            STR_ESC
        ],
    };

    const STRING = {
        className: 'string',
        contains: [ STR_ESC ],
        variants: [
            SINGLE_QUOTE_STR,
            DOUBLE_QUOTE_STR,
        ]
    };

    return {
        name: 'tr-lang',
        aliases: ['trl'],
        unicodeRegex: true,
        keywords: KEYWORDS,
        contains: [
            STRING,
            COMMON_OPS,
            NUMBER,
            LINE_COMMENT,
            BLOCK_COMMENT,
        ],
    };
}
