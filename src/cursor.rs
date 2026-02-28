use firefly_rust::*;

const BOX_ML: i32 = 16;
const BOX_MR: i32 = BOX_ML;
const BOX_MT: i32 = 16;
const LINE_M: i32 = 4;

pub fn draw_cursor(pos: u32, theme: Theme, font: &Font, pressed: bool) {
    let line_h = i32::from(font.char_height()) + LINE_M;
    let y = BOX_MT + pos.cast_signed() * line_h + 1;
    let mut point = Point::new(BOX_ML, y);
    let bbox = Size::new(WIDTH - BOX_ML - BOX_MR, line_h);
    let corner = Size::new(4, 4);

    if pressed {
        point.x += 1;
        point.y += 1;
    } else {
        let style = Style::solid(theme.primary);
        let shadow_point = Point::new(point.x + 1, point.y + 1);
        draw_rounded_rect(shadow_point, bbox, corner, style);
    }

    let style = Style {
        fill_color: theme.bg,
        stroke_color: theme.primary,
        stroke_width: 1,
    };
    draw_rounded_rect(point, bbox, corner, style);
}
