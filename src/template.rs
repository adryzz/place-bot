use std::slice;

use crate::pixel::Color;

pub async fn image_to_template(file: &str) -> anyhow::Result<Vec<(i32, i32, Color)>> {
    let file = tokio::fs::read(file).await?;

    let mut template: Vec<(i32, i32, Color)> = vec![];

    let decoder = png::Decoder::new(&file[..]);

    let mut reader = decoder.read_info()?;
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];

    let info = reader.next_frame(&mut buf).unwrap();

    let bytes = &buf[..info.buffer_size()];

    let pixels = convert(&bytes);

    for i in 0..pixels.len() {
        let color = hex_to_color(pixels[i]);
        let x = (i as u32 % info.width) as i32;
        let y = (i as u32 / info.height) as i32;
        if let Some(c) = color {
            template.push((x, y, c));
        }
    }

    Ok(template)
}

pub fn convert<'a>(data: &'a [u8]) -> &'a [u32] {
    unsafe { &mut slice::from_raw_parts(data.as_ptr() as *const u32, data.len() / 4) }
}

fn hex_to_color(hex: u32) -> Option<Color> {
    match hex {
        4278207999 => Some(Color::Red),
        4278233343 => Some(Color::Orange),
        4281718527 => Some(Color::Yellow),
        4285047552 => Some(Color::DarkGreen),
        4283886974 => Some(Color::LightGreen),
        4288958500 => Some(Color::DarkBlue),
        4293562422 => Some(Color::Blue),
        4294240593 => Some(Color::LightBlue),
        4288618113 => Some(Color::DarkPurple),
        4290792116 => Some(Color::Purple),
        4289370623 => Some(Color::LightPink),
        4280707484 => Some(Color::Brown),
        4278190080 => Some(Color::Black),
        4287663497 => Some(Color::Gray),
        4292466644 => Some(Color::LightGray),
        4294967295 => Some(Color::White),
        _ => None,
    }
}
