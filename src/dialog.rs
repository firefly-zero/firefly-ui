use firefly_rust::*;

#[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn draw_dialog(theme: Theme, font: &Font<'_>, prompt: &str, options: &[&str], cursor: u8) {
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
            // ...
        }
        // Two options, draw both on the same line.
        2 => {
            // ...
        }
        // Many options, draw them in one column.
        _ => {
            // ...
        }
    }
}
