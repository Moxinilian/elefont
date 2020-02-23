use super::*;

use rusttype::{GlyphId, Font, Point, Scale};

impl FontProvider for Font<'_> {
    fn line_width(&self, _size: f32) -> f32 {
        0.0
    }

    fn line_height(&self, size: f32) -> f32 {
        let metrics = self.v_metrics(Scale { x: size, y: size });

        metrics.ascent - metrics.descent + metrics.line_gap
    }

    fn supports_vertical(&self) -> bool {
        false
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Alpha
    }

    fn glyphs(&self, string: &str, glyphs: &mut Vec<Glyph>) {
        glyphs.extend(string.chars().map(|c| Glyph(self.glyph(c).id().0)));
    }

    fn metrics(&self, key: GlyphKey) -> Metrics {
        let glyph = scaled_glyph(self, key);
        let h_metrics = glyph.h_metrics();
        let shape = glyph
            .positioned(Point { x: 0.0, y: 0.0 })
            .pixel_bounding_box()
            .expect("The size of the glyph could not be calculated");

        Metrics {
            x: shape.min.x,
            y: shape.min.y,
            width: shape.width() as u32,
            height: shape.height() as u32,
            bearing_x: h_metrics.left_side_bearing,
            advance_x: h_metrics.advance_width,
            bearing_y: 0.0,
            advance_y: 0.0,
        }
    }

    fn rasterize(&self, key: GlyphKey) -> Vec<u8> {
        let glyph = scaled_glyph(self, key).positioned(Point { x: 0.0, y: 0.0 });
        let bounds = glyph
            .pixel_bounding_box()
            .expect("The size of the glyph could not be calculated");
        let mut buffer = vec![0u8; (bounds.width() * bounds.height()) as usize];
        let width = bounds.width() as u32;
        glyph.draw(|x, y, val| buffer[(x + y * width) as usize] = (val * 255.0) as u8);

        buffer
    }

    fn kerning(&self, a: Glyph, b: Glyph, size: f32) -> Option<f32> {
        Some(self.pair_kerning(Scale { x: size, y: size }, GlyphId(a.0), GlyphId(b.0)))
    }
}

fn scaled_glyph<'a>(font: &'a Font, key: GlyphKey) -> rusttype::ScaledGlyph<'a> {
    let id = GlyphId(key.glyph.0);
    let glyph = font.glyph(id);
    let size = key.size();
    glyph.scaled(Scale { x: size, y: size })
}
