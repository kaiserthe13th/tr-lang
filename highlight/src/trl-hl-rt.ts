function copyToClipboard(
    ele: HTMLElement,
    msgEle?: HTMLElement
): boolean {
    let result = false;
    navigator.clipboard
        .writeText(ele.innerText)
        .then(_ => {
            if (msgEle) {
                msgEle.innerHTML = 'Copied!';
                setTimeout(() => {
                    msgEle.innerHTML = 'Copy';
                }, 5000);
            }
            result = true;
        })
        .catch(_ => {
            if (msgEle) {
                msgEle.innerHTML = 'Failed to Copy';
                setTimeout(() => {
                    msgEle.innerHTML = 'Copy';
                }, 5000);
            }
        });
    return result;
}

function loadTheme(path_to_theme: string) {
    var theme: HTMLLinkElement = <HTMLLinkElement>document.getElementById('theme-trl-hl');

    if (theme) {
        theme.href = path_to_theme;
    }
}
