console.log('Test Script Started');

console.log('Setting up highlight.js with tr-lang package...');
// highlight.js setup
import hljs from 'highlight.js/lib/core';
import trl from '../src/tr-lang.js';

hljs.registerLanguage('tr-lang', trl);

const CODE = `12.321 de '\\n' de -* A Block
Comment *-
işlev printLn -> a
    a de '\\n' de
son
# Line Comment 1`;

// server setup
console.log('Setting up express server...');
import express from 'express';

const app = express();
const port = 3000;

const RES = `<html>
    <head>
        <title> highlight.js test for tr-lang </title>
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.3.1/styles/monokai.min.css">
        <style>
        div {
            border-radius: 8px;
            padding: 8px;
            overflow: hidden;
            margin: 0;
        }
        pre {
            margin: 0;
        }
        .cblock {
            background-color: #0d1117;
            color: #c9d1d9;
        }
        .hljs {
            background-color: #161b22;
        }
        code {
            font-family: 'Jetbrains Mono', 'Fira Code', 'Ubuntu Mono', monospace;
            font-size: var(--c-font-size);
        }
    </style>

    <style id="cfont-size-t">
    </style>
    <script>
        var strFamily = 'Ubuntu Mono';
        var objDiv = document.createElement('div');
        objDiv.style.fontFamily = strFamily;
        objDiv.appendChild(document.createTextNode('FONT TEST'));
        if (window.getComputedStyle ?
                window.getComputedStyle(objDiv, null).getPropertyValue('font-family') == strFamily :
                objDiv.currentStyle.fontFamily == strFamily) {
            document.getElementById('cfont-size-t').innerHTML = ':root { --c-font-size: 16px; }';
        } else {
            document.getElementById('cfont-size-t').innerHTML = ':root { --c-font-size: 14px; }';
        }
    </script>
    </head>
    <body>
        <div class="cblock">
            Test
            <div class="hljs">
                <pre class="hljs"><code class="hljs-code lang-tr-lang">${
                    hljs.highlightAuto(CODE).value
                }</code></pre>
            </div>
        </div>
    </body>
</html>`;

app.get('/', (req, res) => {
    res.send(RES);
});

app.listen(port, () => {
    console.log(`Listening for requests at http://localhost:${port}...`);
});
