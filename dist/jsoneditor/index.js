window.onload = function() {
    const container = document.getElementById('jsoneditor')
    const options = {
        // modes: ['text', 'code', 'tree', 'form', 'view'],
        modes: ['text', 'code', 'tree'],
        mode: 'code',
        ace: ace
    }
    const editor = new JSONEditor(container, options)
    editor.setText('')
}