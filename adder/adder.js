fetch('./add.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes))
  .then(results => {
    setEventListener(results.instance);
  })
  .catch(console.error);

function setEventListener(wasm) {
  const lhs = document.getElementById('lhs');
  const rhs = document.getElementById('rhs');

  const adder = () => {
    document.getElementById('sum').textContent = wasm.exports.add(
      parseInt(lhs.value),
      parseInt(rhs.value),
    );
  };

  lhs.addEventListener('change', adder);
  rhs.addEventListener('change', adder);
}
