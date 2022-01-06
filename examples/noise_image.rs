use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    let w = 128usize;
    let h = 128usize;
    let s = 0.1f32;

    let mut noise = Vec::with_capacity(w * h);

    let mut max = std::u8::MIN;
    let mut min = std::u8::MAX;

    for y in 0..h {
        for x in 0..w {
            let sample =
                (simplex_noise::noise3(x as f32 * s, y as f32 * s, 0.0) * 127.0 + 128.0) as u8;
            max = sample.max(max);
            min = sample.min(min);
            noise.push(sample);
        }
    }

    // println!("{:?}", noise);
    // println!("{} .. {}", min, max);

    let png_path = Path::new("./noise_image.png");
    let png_file = File::create(png_path).unwrap();
    let png_file_writer = BufWriter::new(png_file);

    let mut encoder = png::Encoder::new(png_file_writer, w as u32, h as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&noise).unwrap();
}
