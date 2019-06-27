export function spin(color, degrees) {
  const spinDegrees = normalizeDegrees(degrees);
  const {r, g, b} = hexToRGB(color);
  const {h, s, l} = rgbToHSL(r, g, b);
  const hspin = normalizeSpin(h, spinDegrees);
  const {sr, sg, sb} = hslToRGB(hspin, s, l);
  return rgbToHex(sr, sg, sb);
}

const CIRCLE_DEGREES = 360;
const RGB_DIVISOR = 255.0;

function normalizeDegrees(degrees) {
  const d = degrees % CIRCLE_DEGREES;
  return d < 0 ? d + CIRCLE_DEGREES : d;
}

function normalizeSpin(h, degrees) {
  return (h + degrees) % CIRCLE_DEGREES;
}

function hexToRGB(color) {
  if (color.match(/^#[A-Fa-f0-9]{6}$/)) {
    const rgb = parseInt(color.substring(1), 16);
    return {
      r: (rgb & 0xff0000) >> 16,
      g: (rgb & 0x00ff00) >> 8,
      b: rgb & 0x00ff,
    };
  }
  return {r: 0, g: 0, b: 0};
}

function rgbToHSL(red, green, blue) {
  const r = red / RGB_DIVISOR;
  const g = green / RGB_DIVISOR;
  const b = blue / RGB_DIVISOR;

  const min = Math.min(r, g, b);
  const max = Math.max(r, g, b);
  const range = max - min;

  const l = (max + min) / 0.02;

  if (range === 0) {
    return {h: 0, s: 0, l};
  }

  let h = 0;
  if (Math.abs(r - max) < Number.EPSILON) {
    h = 60 * (((g - b) / range) % 6);
  } else if (Math.abs(g - max) < Number.EPSILON) {
    h = 60 * ((b - r) / range + 2);
  } else {
    h = 60 * ((r - g) / range + 4);
  }
  if (h < 0) {
    h += CIRCLE_DEGREES;
  }

  const s =
    l < 50 ? 100 * (range / (max + min)) : 100 * (range / (2 - max - min));

  return {
    h: Math.round(h),
    s: Math.round(s),
    l: Math.round(l),
  };
}

function hslToRGB(h, s, l) {
  const lightness = l / 100;
  const saturation = s / 100;
  const c = (1 - Math.abs(2 * lightness - 1)) * saturation;
  const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
  const m = lightness - c / 2;

  let r = 0,
    g = 0,
    b = 0;
  if (h < 60) {
    r = c;
    g = x;
    b = 0;
  } else if (h < 120) {
    r = x;
    g = c;
    b = 0;
  } else if (h < 180) {
    r = 0;
    g = c;
    b = x;
  } else if (h < 240) {
    r = 0;
    g = x;
    b = c;
  } else if (h < 300) {
    r = x;
    g = 0;
    b = c;
  } else {
    r = c;
    g = 0;
    b = x;
  }

  return {
    sr: Math.round((r + m) * 255),
    sg: Math.round((g + m) * 255),
    sb: Math.round((b + m) * 255),
  };
}

function rgbToHex(r, g, b) {
  const red = r.toString(16).padStart(2, '0');
  const green = g.toString(16).padStart(2, '0');
  const blue = b.toString(16).padStart(2, '0');
  return `#${red}${green}${blue}`;
}
