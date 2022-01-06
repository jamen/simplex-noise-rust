// const BIT_PATTERNS: [u8; 8] = [0x15, 0x38, 0x32, 0x2c, 0x0d, 0x13, 0x07, 0x2a];

// #[rustfmt::skip]
// #[inline]
// fn gradient_idx(i: i32, j: i32, k: i32) -> u8 {
//     b(i, j, k, 0x01) + b(j, k, i, 0x02) + b(k, i, j, 0x04) + b(i, j, k, 0x08) +
//     b(j, k, i, 0x10) + b(k, i, j, 0x20) + b(i, j, k, 0x40) + b(j, k, i, 0x80)
// }

// #[inline]
// fn b(i: i32, j: i32, k: i32, w: i32) -> u8 {
//     BIT_PATTERNS[(4 * (i & w) + 2 * (j & w) + (k & w)) as usize]
// }

// const MAGNITUDE_BITS: u8 = 0b00000111;
// const OCTANT_BITS: u8 = 0b00111000;

// fn gradient(x: i32, y: i32, z: i32) {
//     let idx = gradient_idx(x, y, z);

//     let (p, q, r) = match MAGNITUDE_BITS & idx {
//         0b00000000 => (x, y, z),
//         0b00000001 => (y, z, 0),
//         0b00000010 => (z, x, 0),
//         0b00000011 => (x, y, 0),

//         0b00000100 => (x, y, z),
//         0b00000101 => (y, 0, x),
//         0b00000110 => (z, 0, y),
//         0b00000111 => (x, 0, z),

//         _ => unreachable!(),
//     };

//     #[rustfmt::skip]
//     let octant = match OCTANT_BITS & idx {
//         0b00000000 => -p-q+r,
//         0b00001000 =>  p-q-r,
//         0b00010000 => -p+q-r,
//         0b00011000 =>  p+q+r,

//         0b00100000 =>  p+q-r,
//         0b00101000 => -p+q+r,
//         0b00110000 =>  p-q+r,
//         0b00111000 => -p-q-r,

//         _ => unreachable!(),
//     };
// }

// fn skew(x: f32, y: f32, z: f32) -> [f32; 3] {
//     let s = (x + y + z) / 3.0;
//     [x + s, y + s, z + s]
// }

// fn corner(sx: f32, sy: f32, sz: f32) -> [i32; 3] {
//     [sx.floor() as i32, sy.floor() as i32, sz.floor() as i32]
// }

// fn unskew(i: f32, j: f32, k: f32) -> [f32; 3] {
//     let s = (i + j + k) / 6.0;
//     [i - s, j - s, k - s]
// }

#[rustfmt::skip]
pub fn noise3(x: f32, y: f32, z: f32) -> f32 {
    let s = (x + y + z) / 3.0;
    let (i, j, k) = ((x + s).floor() as i32, (y + s).floor() as i32, (z + s).floor() as i32);

    let s = (i + j + k) as f32 / 6.0;
    let (u, v, w) = (x - i as f32 + s, y - j as f32 + s, z - k as f32 + s);

    let hi = if u >= w { if u >= v { 0 } else { 1 } } else { if v >= w { 1 } else { 2 } };
    let lo = if u <  w { if u <  v { 0 } else { 1 } } else { if v <  w { 1 } else { 2 } };

    let mut k = Noise3 { a: [0, 0, 0], ijk: [i, j, k], uvw: [u, v, w] };

    k.part(hi) + k.part(3 - hi - lo) + k.part(lo) + k.part(0)
}

struct Noise3 {
    a: [i32; 3],
    ijk: [i32; 3],
    uvw: [f32; 3],
}

impl Noise3 {
    #[rustfmt::skip]
    fn part(&mut self, a: i32) -> f32 {
        let s = (self.a[0] + self.a[1] + self.a[2]) as f32 / 6.0;

        let x = self.uvw[0] - self.a[0] as f32 + s;
        let y = self.uvw[1] - self.a[1] as f32 + s;
        let z = self.uvw[2] - self.a[2] as f32 + s;

        let mut t = 0.6 - x * x - y * y - z * z;

        let [i, j, k] = self.ijk;
        let [a1, a2, a3] = self.a;

        let h = shuffle(i + a1, j + a2, k + a3);

        self.a[a as usize] += 1;

        if h < 0 {
            return 0.0;
        }

        let b5 = h >> 5 & 1;
        let b4 = h >> 4 & 1;
        let b3 = h >> 3 & 1;
        let b2 = h >> 2 & 1;
        let b = h & 3;

        let p = match b { 1 => x, 2 => y, _ => z };
        let q = match b { 1 => y, 2 => z, _ => x };
        let r = match b { 1 => z, 2 => x, _ => y };

        let p = if b5 == b3 { -p } else { p };
        let q = if b5 == b4 { -q } else { q };
        let r = if b5 != (b4 ^ b3) { -r } else { r };

        t *= t;

        8.0 * t * t * (p + if b == 0 { q + r } else { if b2 == 0 { q } else { r } })
    }
}

const BIT_PATTERNS: [i32; 8] = [0x15, 0x38, 0x32, 0x2c, 0x0d, 0x13, 0x07, 0x2a];

#[rustfmt::skip]
#[inline]
fn shuffle(i: i32, j: i32, k: i32) -> i32 {
    b(i, j, k, 0) + b(j, k, i, 1) + b(k, i, j, 2) + b(i, j, k, 3) +
    b(j, k, i, 4) + b(k, i, j, 5) + b(i, j, k, 6) + b(j, k, i, 7)
}

#[inline]
fn b(i: i32, j: i32, k: i32, w: i32) -> i32 {
    BIT_PATTERNS[((i >> w & 1) << 2 | (j >> w & 1) << 1 | (k >> w & 1)) as usize]
}
