export const hslToRgb = (hue: number, sat: number, lum: number): [number, number, number] => {
    const s = sat / 100;
    const l = lum / 100;

    const c = (1 - Math.abs(2 * l - 1)) * s,
        x = c * (1 - Math.abs((hue / 60) % 2 - 1)),
        m = l - c / 2;
    let r = 0,
        g = 0,
        b = 0;
    
    if (0 <= hue && hue < 60) {
        r = c; g = x; b = 0;  
    } else if (60 <= hue && hue < 120) {
        r = x; g = c; b = 0;
    } else if (120 <= hue && hue < 180) {
        r = 0; g = c; b = x;
    } else if (180 <= hue && hue < 240) {
        r = 0; g = x; b = c;
    } else if (240 <= hue && hue < 300) {
        r = x; g = 0; b = c;
    } else if (300 <= hue && hue < 360) {
        r = c; g = 0; b = x;
    }

    r = Math.round((r + m) * 255);
    g = Math.round((g + m) * 255);
    b = Math.round((b + m) * 255);

    return [r, g, b];
}

export const rgbToHexString = (r: number, g: number, b: number): string => {
    const rHex = r.toString(16), gHex = g.toString(16), bHex = b.toString(16);
    return "#" + rHex + gHex + bHex;
}