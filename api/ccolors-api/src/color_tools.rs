// def rgb_to_hsv(r, g, b):
//     maxc = max(r, g, b)
//     minc = min(r, g, b)
//     v = maxc
//     if minc == maxc:
//         return 0.0, 0.0, v
//     s = (maxc-minc) / maxc
//     rc = (maxc-r) / (maxc-minc)
//     gc = (maxc-g) / (maxc-minc)
//     bc = (maxc-b) / (maxc-minc)
//     if r == maxc:
//         h = bc-gc
//     elif g == maxc:
//         h = 2.0+rc-bc
//     else:
//         h = 4.0+gc-rc
//     h = (h/6.0) % 1.0
//     return h, s, v

/// see https://www.rapidtables.com/convert/color/rgb-to-hsv.html
pub fn rgb_to_hsv(red: f32, green: f32, blue: f32) -> (f32, f32, f32) {
    let c_max = *[red, green, blue]
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap_or(&0.0);
    let c_min = *[red, green, blue]
        .iter()
        .min_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap_or(&0.0);

    log::info!("c_max: {}", c_max);
    log::info!("c_min: {}", c_min);

    let hue_angle;
    let val = c_max;

    let delta = c_max - c_min;

    const EPSILON: f32 = 1e-4;

    if delta.abs() <= EPSILON {
        return (0.0, 0.0, val);
    }

    let sat;
    if c_max < EPSILON {
        sat = 0.0f32;
    } else {
        sat = delta / c_max;
    }

    if (c_max - red).abs() < EPSILON {
        hue_angle = 60.0 * (((green - blue) / delta) % 6.0);
    } else if (c_max - green).abs() < EPSILON {
        hue_angle = 60.0 * ((blue - red) / delta + 2.0);
    } else {
        hue_angle = 60.0 * ((red - green) / delta + 4.0);
    }

    let hue;
    if hue_angle < 0.0 {
        hue = (hue_angle + 360.0) / 360.0;
    } else {
        hue = hue_angle / 360.0;
    }

    log::info!("HSV: {:?}", (hue, sat, val));

    (hue, sat, val)
}
