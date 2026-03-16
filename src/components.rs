use firefly_rust::*;

const BOX_ML: i32 = 16;
const BOX_MR: i32 = BOX_ML;
const BOX_MT: i32 = 16;
const LINE_M: i32 = 4;

const CURSOR_ML: i32 = 4;
const CURSOR_X: i32 = BOX_ML + CURSOR_ML;

pub fn draw_cursor(pos: u32, theme: Theme, font: &Font, pressed: bool, jitter: i8) {
    let line_h = i32::from(font.char_height()) + LINE_M;
    let jitter = if pressed { 0 } else { jitter };
    let y = BOX_MT + pos.cast_signed() * line_h + 1 + i32::from(jitter);
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

pub fn draw_switch(idx: i32, is_on: bool, pressed: bool, font: &Font, theme: Theme) {
    let font_h = i32::from(font.char_height());
    let x = WIDTH - CURSOR_X - font_h * 2;
    let line_h = font_h + LINE_M;
    let y = CURSOR_X + idx * line_h - 1;
    let mut point = Point::new(x, y);
    if pressed {
        point.x += 1;
        point.y += 1;
    }

    {
        let mut switch_point = point;
        let color = if is_on {
            switch_point.x += font_h;
            theme.accent
        } else {
            theme.secondary
        };
        let style = Style::solid(color);
        draw_circle(switch_point, font_h, style);
    }

    let style = Style::outlined(theme.primary, 1);
    let size = Size::new(font_h * 2, font_h);
    let corner = Size::new(font_h / 2, font_h / 2);
    draw_rounded_rect(point, size, corner, style);
}
