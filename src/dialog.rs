use firefly_rust::*;

#[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn draw_dialog<F: Font>(
    theme: Theme,
    font: &F,
    prompt: &str,
    options: &[&str],
    cursor: u8,
    pressed: bool,
) {
    // Calculate the box size and margins.
    let horiz = options.len() == 2 && options[0].len() <= 6 && options[1].len() <= 6;
    let max_chars = if horiz {
        options[0].len() + options[1].len()
    } else {
        options.iter().map(|t| t.len()).max().unwrap_or_default()
    };
    let max_chars = max_chars.max(prompt.len());
    let width = i32::from(font.char_width()) * max_chars as i32 + 8;
    let left = (WIDTH - width) / 2;

    let n_lines = if horiz { 2 } else { options.len() + 1 };
    let line_height = i32::from(font.char_height()) + 4;
    let height = line_height * n_lines as i32 + 4;
    let top = (HEIGHT - height) / 2;

    // Draw the box.
    let size = Size::new(width, height);
    draw_rounded_rect(
        Point::new(left + 1, top + 1),
        size,
        Size::new(4, 4),
        Style::solid(theme.primary),
    );
    draw_rounded_rect(
        Point::new(left, top),
        size,
        Size::new(4, 4),
        Style {
            fill_color: theme.bg,
            stroke_color: theme.primary,
            stroke_width: 1,
        },
    );

    // Draw the prompt message.
    let point = Point::new(
        (WIDTH - font.line_width_utf8(prompt).cast_signed()) / 2,
        top + line_height - 4,
    );
    draw_text(prompt, font, point, theme.accent);

    // Draw two options on the same line (typically no/yes, back/forward, etc.).
    if horiz {
        let text_y = top + line_height * 2 - 4;
        let cursor_y = text_y - line_height + 5;
        let cursor_width = WIDTH / 2 - left - 8;

        // Left.
        {
            let text = options[0];
            let offset = ((WIDTH / 2 - left) - font.line_width_utf8(text).cast_signed()) / 2;
            if cursor == 0 {
                let point = Point::new(left + 4, cursor_y);
                draw_cursor(point, cursor_width, theme, font, pressed);
            }
            let mut point = Point::new(left + offset, text_y);
            if cursor == 0 && pressed {
                point.x += 1;
                point.y += 1;
            }
            draw_text(text, font, point, theme.primary);
        }

        // Right.
        {
            let text = options[1];
            let offset = ((WIDTH / 2 - left) - font.line_width_utf8(text).cast_signed()) / 2;
            if cursor > 0 {
                let point = Point::new(WIDTH / 2 + 4, cursor_y);
                draw_cursor(point, cursor_width, theme, font, pressed);
            }
            let mut point = Point::new(WIDTH / 2 + offset, text_y);
            if cursor > 0 && pressed {
                point.x += 1;
                point.y += 1;
            }
            draw_text(text, font, point, theme.primary);
        }
        return;
    }

    // Draw the options
    match options.len() {
        // No options.
        0 => {}
        // One option, make it centered.
        1 => {
            let text = options[0];
            let point = Point::new(left + 3, top + line_height + 1);
            draw_cursor(point, width - 6, theme, font, pressed);
            let mut point = Point::new(
                (WIDTH - font.line_width_utf8(text).cast_signed()) / 2,
                top + line_height * 2 - 4,
            );
            if pressed {
                point.x += 1;
                point.y += 1;
            }
            draw_text(text, font, point, theme.primary);
        }
        // Many options, draw them in one column.
        _ => {
            for (text, i) in options.iter().zip(0..) {
                let mut point = Point::new(left + 4, top + line_height * (i + 2) - 4);
                if i == i32::from(cursor) {
                    let cursor_point = Point::new(point.x - 2, point.y - 8);
                    if pressed {
                        point.x += 1;
                        point.y += 1;
                    }
                    draw_cursor(cursor_point, width - 5, theme, font, pressed);
                }
                draw_text(text, font, point, theme.primary);
            }
        }
    }
}

fn draw_cursor<F: Font>(mut point: Point, width: i32, theme: Theme, font: &F, pressed: bool) {
    let bbox = Size::new(width, i32::from(font.char_height()) + 4);
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
