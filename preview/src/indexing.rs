use rayon::prelude::*;
use rvc_shared::{
    colors::{FloatColor, IntColor},
    dmatrix::DitherMatrix,
    palette::Palette,
    plane::Plane,
};

pub fn convert_posterize(in_image: &Plane<IntColor>, out_image: &mut Plane<i32>, palette: &Palette) {
    for (in_pixel, out_pixel) in in_image.data.iter().zip(out_image.data.iter_mut()) {
        *out_pixel = palette.find(FloatColor::from(in_pixel));
    }
}

fn add_error(plane: &mut Plane<FloatColor>, x: u32, y: u32, error: f64) {
    let prev = plane.get(x, y);
    plane.set(x, y, prev + error);
}

pub fn convert_fs(in_image: &Plane<IntColor>, out_image: &mut Plane<i32>, palette: &Palette) {
    let mut inner = Plane::new(in_image.width, in_image.height, FloatColor::BLACK);
    for y in 0..out_image.height {
        for x in 0..out_image.width {
            let old_color = FloatColor::from(in_image.get(x, y)) + inner.get(x, y);
            let new_color_index = palette.find(old_color);
            let new_color = palette.get(new_color_index);
            out_image.set(x, y, new_color_index);
            let error = (old_color.r - new_color.r + old_color.g - new_color.g + old_color.b - new_color.b) / 3.0;
            if x < out_image.width - 1 {
                add_error(&mut inner, x + 1, y, error * 7.0 / 16.0);
            }
            if y < out_image.height - 1 {
                if x > 0 {
                    add_error(&mut inner, x - 1, y + 1, error * 3.0 / 16.0);
                }
                add_error(&mut inner, x, y + 1, error * 5.0 / 16.0);
                if x < out_image.width - 1 {
                    add_error(&mut inner, x + 1, y + 1, error * 1.0 / 16.0);
                }
            }
        }
    }
}

const DITHER_TRESHOLD: f64 = 0.5;

pub fn convert_matrix(
    in_image: &Plane<IntColor>,
    out_image: &mut Plane<i32>,
    palette: &Palette,
    matrix: &DitherMatrix,
) {
    out_image.data.par_iter_mut().enumerate().for_each(|(i, pixel)| {
        let x = i as u32 % out_image.width;
        let y = i as u32 / out_image.width;
        let mut candidates = vec![0i32; matrix.levels as usize];
        let mut color_error = FloatColor::BLACK;

        let in_color = FloatColor::from(in_image.get(x, y));
        for cand in candidates.iter_mut() {
            let attempt = (in_color + color_error * DITHER_TRESHOLD).clip();
            let index = palette.find(attempt);
            *cand = index;
            let new_color = palette.get(index);
            color_error += in_color - new_color;
        }
        candidates.sort();
        *pixel = candidates[matrix.get(x, y) as usize];
    });
}
