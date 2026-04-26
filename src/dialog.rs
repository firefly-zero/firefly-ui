use firefly_rust::*;

#[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn draw_dialog(
    theme: Theme,
    font: &Font<'_>,
    prompt: &str,
    options: &[&str],
    cursor: u8,
    pressed: bool,
) {
    // Calculate the box size and margins.
    let max_chars = if options.len() == 2 {
        options[0].len() + options[1].len()
    } else {
        options.iter().map(|t| t.len()).max().unwrap_or_default()
    };
    let max_chars = max_chars.max(prompt.len());
    let width = i32::from(font.char_width()) * max_chars as i32;
    let left = (WIDTH - width) / 2;

    let n_lines = options.len() + 1;
    let height = (i32::from(font.char_height()) + 2) * n_lines as i32;
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
    let line_height = i32::from(font.char_height());
    let point = Point::new(
        (WIDTH - font.line_width_utf8(prompt).cast_signed()) / 2,
        top + line_height,
    );
    draw_text(prompt, font, point, theme.accent);

    // Draw the options
    match options.len() {
        // No options.
        0 => {}
        // One option, make it centered.
        1 => {
            let text = options[0];
            let point = Point::new(
                (WIDTH - font.line_width_utf8(text).cast_signed()) / 2,
                top + line_height * 2,
            );
            draw_text(text, font, point, theme.primary);
            draw_cursor(point, width, theme, font, pressed);
        }
        // Two options, draw both on the same line.
        2 => {
            let text = options[0];
            let offset = ((WIDTH / 2 - left) - font.line_width_utf8(text).cast_signed()) / 2;
            let point = Point::new(left + offset, top + line_height * 2);
            draw_text(text, font, point, theme.primary);
            if cursor == 0 {
                draw_cursor(point, WIDTH / 2 - offset, theme, font, pressed);
            }

            let text = options[1];
            let offset = ((WIDTH / 2 - left) - font.line_width_utf8(text).cast_signed()) / 2;
            let point = Point::new(WIDTH - offset, top + line_height * 2);
            draw_text(text, font, point, theme.primary);
            if cursor > 0 {
                draw_cursor(point, WIDTH / 2 + offset, theme, font, pressed);
            }
        }
        // Many options, draw them in one column.
        _ => {
            for (text, i) in options.iter().zip(0..) {
                let point = Point::new(left + 4, top + line_height * 2);
                draw_text(text, font, point, theme.primary);
                if i == cursor {
                    draw_cursor(point, width, theme, font, pressed);
                }
            }
        }
    }
}

fn draw_cursor(mut point: Point, width: i32, theme: Theme, font: &Font, pressed: bool) {
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
