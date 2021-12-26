"use strict";
const codes = document.getElementsByClassName('code');
for (let i = 0; i < codes.length; i++) {
    codes[i].innerHTML = new Highlighter(codes[i].innerText).highlight();
}
