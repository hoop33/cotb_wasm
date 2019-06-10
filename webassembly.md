footer: https://github.com/hoop33/cotb_wasm
slidenumbers: true

# Why the Web Wants WebAssembly

## Rob Warner

* GitHub: @hoop33
* Twitter: @hoop33
* Email: hoop33@gmail.com
* Blog: https://grailbox.com

---

## What Is WebAssembly?

> WebAssembly ... is a binary instruction format for a stack-based virtual machine.

https://webassembly.org

---

## WebAssembly Features

* Stack-based
* Portable
* Hosted

---

## Sound Familiar?

![](./images/duke.png)
![](./images/silverlight.png)
![](./images/adobe-air.png)
![](./images/adobe-flash.jpg)

---

## How Is WebAssembly Different?

* Multiple Language Support
* Sandboxed
* Integrated
* Small binaries
* Competition

---

## Tools

* WebAssembly Studio
  * https://webassembly.studio/
* The WebAssembly Binary Toolkit 
  * https://github.com/WebAssembly/wabt

---

## WASM Data Types

* `i32`
* `i64`
* `f32`
* `f64`

---

## First Project: Adder

* WASM function to add two integers
* Page to input numbers and display sum
* JavaScript to load module and call function

---

## Add Function

```lisp
(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    get_local $lhs
    get_local $rhs
    i32.add)
  (export "add" (func $add))
)
```

---

## HTML

```html
<input type="number" id="lhs" min="0" value="0">
+
<input type="number" id="rhs" min="0" value="0">
=
<span id="sum">0</span>
```

---

## JavaScript

```javascript
fetch('./add.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes))
  .then(results => {
    instance = results.instance;
    // Set event listener here
  })
  .catch(console.error);
```

---

## JavaScript (cont)

```javascript
const lhs = document.getElementById('lhs');
const rhs = document.getElementById('rhs');

const adder = function() {
  document.getElementById('sum').textContent = instance.exports.add(
    parseInt(lhs.value),
    parseInt(rhs.value),
  );
};

lhs.addEventListener('change', adder);
rhs.addEventListener('change', adder);
```

---

## Further Reading

* https://webassembly.org
* _Programming WebAssembly with Rust_ by Kevin Hoffman
* https://github.com/appcypher/awesome-wasm-langs

---

## Rob Warner

* GitHub: @hoop33
* Twitter: @hoop33
* Email: hoop33@gmail.com
* Blog: https://grailbox.com
* Repo: https://github.com/hoop33/cotb_wasm

---

