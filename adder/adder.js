fetch('./add.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes))
  .then(results => {
    instance = results.instance;

    const adder = function() {
      const lhs = parseInt(document.getElementById('lhs').value);
      const rhs = parseInt(document.getElementById('rhs').value);
      document.getElementById('sum').textContent = instance.exports.add(
        lhs,
        rhs,
      );
    };

    document.getElementById('lhs').addEventListener('change', adder);
    document.getElementById('rhs').addEventListener('change', adder);
  })
  .catch(console.error);
