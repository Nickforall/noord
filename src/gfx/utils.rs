pub fn gl_to_pos(position: (i32, i32), dimensions: (u32, u32)) -> [f32; 2] {
    let (pixel_x, pixel_y) = position;
    let (d_width, d_height) = dimensions;

    let factor = 2.0;
    let x = -1.0 + (factor / d_width as f32) * pixel_x as f32;
    let y = 1.0 - (factor / d_height as f32) * pixel_y as f32;

    [x, y]
}
