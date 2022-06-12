document.focusElement = (element) => {
    document.activeElement = element;
    element.focus()
    console.log(document.activeElement)
}

document.oncontextmenu = (e) => e.preventDefault()