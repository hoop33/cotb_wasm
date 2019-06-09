fetch('./add.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes))
  .then(results => {
    const instance = results.instance;

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
  })
  .catch(console.error);
