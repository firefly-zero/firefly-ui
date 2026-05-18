#![no_std]
#![no_main]
use core::mem::MaybeUninit;
use firefly_rust::*;
use firefly_ui::*;

static mut STATE: MaybeUninit<State> = MaybeUninit::uninit();

pub struct State {
    font: FileBuf,
    btns: Buttons,
    screen: u8,
}

fn get_state() -> &'static mut State {
    #[allow(static_mut_refs)]
    unsafe {
        STATE.assume_init_mut()
    }
}

#[unsafe(no_mangle)]
extern "C" fn boot() {
    let font = load_file_buf("ascii").unwrap();
    let state = State {
        font,
        btns: Buttons::default(),
        screen: 0,
    };
    #[allow(static_mut_refs)]
    unsafe {
        STATE.write(state)
    };
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    let state = get_state();
    let btns = read_buttons(Peer::COMBINED);
    let released = btns.just_released(&state.btns);
    state.btns = btns;
    if released.e {
        state.screen += 1;
    }
    if released.w {
        state.screen -= 1;
    }

    clear_screen(Color::Yellow);

    let theme = get_settings(Peer::COMBINED).theme;
    match state.screen {
        0 => {
            draw_bg(theme);
            draw_title(
                "what does the fox say???",
                state.btns.any(),
                &state.font.as_font(),
                theme.accent,
            );
            draw_cursor(1, theme, &state.font.as_font(), state.btns.any(), 0);
            draw_switch(1, true, state.btns.any(), &state.font.as_font(), theme);
            draw_switch(2, false, state.btns.any(), &state.font.as_font(), theme);
        }
        1 => draw_dialog(
            theme,
            &state.font.as_font(),
            "sorry man, something went wrong",
            &["okay"],
            0,
            state.btns.any(),
        ),
        2 => draw_dialog(
            theme,
            &state.font.as_font(),
            "huh?",
            &["are you talking to me?????"],
            0,
            state.btns.any(),
        ),
        3 => draw_dialog(
            theme,
            &state.font.as_font(),
            "wanna dance?",
            &["no", "yes"],
            if state.btns.n { 1 } else { 0 },
            state.btns.any(),
        ),
        4 => draw_dialog(
            theme,
            &state.font.as_font(),
            "sure???",
            &["no", "yes", "maybe"],
            if state.btns.n { 1 } else { 0 },
            state.btns.any(),
        ),
        _ => state.screen = 0,
    }
}
