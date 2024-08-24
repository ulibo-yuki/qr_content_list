let itemIndex = 0;

function addItem() {
    const container = document.getElementById('itemContainer');
    const div = document.createElement('div');
    div.className = 'item-container';
    div.className = 'form_title';
    div.id = `item-${itemIndex}`;
    
    const input = document.createElement('input');
    input.type = 'text';
    input.name = 'items[]';
    input.placeholder = '項目を入力';
    input.addEventListener('blur', concat);
    input.addEventListener('input', concat);
    input.addEventListener('keydown', function(event) {
        if (event.key == 'Enter') {
            event.preventDefault();
            concat();
        }
    });
// <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M5 11V13H19V11H5Z"></path></svg>
    // const svg = document.createAttributeNS('http://www.w3.org/2000/svg', 'svg');
    // svg.setAttribute()

    const button = document.createElement('button');
    button.type = 'button';
    button.className = 'remove_btn';
    button.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M5 11V13H19V11H5Z"></path></svg>';
    button.onclick = function() {
        removeItem(div.id);
    };

    div.appendChild(input);
    div.appendChild(button);
    container.appendChild(div);

    itemIndex ++;
    concat();
}

function removeItem(id) {
    const item = document.getElementById(id);
    item.remove();
    concat();
}

function concat() {
    const inputs = document.querySelectorAll('#itemContainer input');
    let concatstring = '';
    for (const input of inputs) {
        concatstring += input.value + ',';
    }
    console.log(concatstring.trim());

    var textarea = document.getElementById('content');
    textarea.value = concatstring;
}