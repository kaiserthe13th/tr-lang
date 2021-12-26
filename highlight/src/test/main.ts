const codes = document.getElementsByClassName('code');

for (let i = 0; i < codes.length; i++) {
    codes[i].innerHTML = new Highlighter((<HTMLElement>codes[i]).innerText).highlight();
}
