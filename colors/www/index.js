import * as colors from 'colors';
import * as jscolors from 'jscolors';

const base = document.getElementById('base');

const triad1 = document.getElementById('triad1');
const triad2 = document.getElementById('triad2');
const triad3 = document.getElementById('triad3');

const jsTriad1 = document.getElementById('jsTriad1');
const jsTriad2 = document.getElementById('jsTriad2');
const jsTriad3 = document.getElementById('jsTriad3');

base.addEventListener('change', event => {
  const bc = event.srcElement.value;

  triad1.style.backgroundColor = bc;
  triad2.style.backgroundColor = colors.spin(bc, 120);
  triad3.style.backgroundColor = colors.spin(bc, 240);

  jsTriad1.style.backgroundColor = bc;
  jsTriad2.style.backgroundColor = jscolors.spin(bc, 120);
  jsTriad3.style.backgroundColor = jscolors.spin(bc, 240);

  const startRust = new Date();
  for (var i = 0; i < 10000; i++) {
    colors.spin(bc, 120);
    colors.spin(bc, 240);
  }
  const endRust = new Date();

  const startJs = new Date();
  for (var i = 0; i < 10000; i++) {
    jscolors.spin(bc, 120);
    jscolors.spin(bc, 240);
  }
  const endJs = new Date();

  console.log(`Rust: ${endRust - startRust}`);
  console.log(`JS: ${endJs - startJs}`);
});
