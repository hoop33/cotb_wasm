import * as colors from 'colors';

const base = document.getElementById('base');

const triad1 = document.getElementById('triad1');
const triad2 = document.getElementById('triad2');
const triad3 = document.getElementById('triad3');

const tetrad1 = document.getElementById('tetrad1');
const tetrad2 = document.getElementById('tetrad2');
const tetrad3 = document.getElementById('tetrad3');
const tetrad4 = document.getElementById('tetrad4');

base.addEventListener('change', event => {
  const bc = event.srcElement.value;

  triad1.style.backgroundColor = bc;
  triad2.style.backgroundColor = colors.spin(bc, 120);
  triad3.style.backgroundColor = colors.spin(bc, 240);

  tetrad1.style.backgroundColor = bc;
  tetrad2.style.backgroundColor = colors.spin(bc, 90);
  tetrad3.style.backgroundColor = colors.spin(bc, 180);
  tetrad4.style.backgroundColor = colors.spin(bc, 270);
});
