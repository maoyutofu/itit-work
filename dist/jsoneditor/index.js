let editor;
window.onload = function() {
    const container = document.getElementById('jsoneditor')
    const options = {
        // modes: ['text', 'code', 'tree', 'form', 'view'],
        modes: ['text', 'code', 'tree'],
        mode: 'code',
        ace: ace
    }
    editor = new JSONEditor(container, options)
    editor.setText('')
}

function format() {
    const jsonObj = JSON.parse(editor.getText())
    const jsonStr = JSON.stringify(jsonObj, undefined, 2)
    editor.setText(jsonStr)
}

function compress() {
    const jsonObj = JSON.parse(editor.getText())
    const jsonStr = JSON.stringify(jsonObj)
    editor.setText(jsonStr)
}

function transfer() {
    const jsonStr = editor.getText();
    const escapedStr = jsonStr.replace(/"/g, '\\"');
    editor.setText(escapedStr)
}

function detransfer() {
    const escapedStr = editor.getText();
    const unescapedStr = escapedStr.replace(/\\"/g, '"');
    editor.setText(unescapedStr)
}

function toStr() {
    const jsonStr = JSON.stringify(editor.getText())
    editor.setText(jsonStr)
}