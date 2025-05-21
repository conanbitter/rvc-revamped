use rvc_shared::{colors::IntColor, palette::Palette, plane::Plane};

pub fn convert_posterize(in_image: &Plane<IntColor>, out_image: &mut Plane<i32>, palette: &Palette) {
    for (in_pixel, out_pixel) in in_image.data.iter().zip(out_image.data.iter_mut()) {
        *out_pixel = palette.find(*in_pixel);
    }
}
