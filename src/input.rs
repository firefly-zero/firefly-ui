use firefly_rust::*;

#[derive(Eq, PartialEq, Copy, Clone, Default)]
pub enum Input {
    Up,
    Down,
    Left,
    Right,
    Select,
    Back,
    #[default]
    None,
}

#[derive(Default)]
pub struct InputManager {
    pub peer: Peer,

    /// For how long up or down button (pad) is held.
    ///
    /// Used for automatic scroll when holding up or down on the pad
    /// and for jittering the cursor at the list boundaries.
    held_for: u32,

    /// The state of buttons on the previous frame.
    old_buttons: Buttons,

    /// The state of direction buttons on the previous frame.
    old_dpad: DPad4,

    /// The cached input value from the last update.
    input: Input,

    /// True if the select is held down.
    ///
    /// Used to animate button press on the cursor.
    pressed: bool,

    /// Ignore button presses.
    ///
    /// Activated when the cursor is moved,
    /// deactivated when all buttons are released.
    /// Allows to cancel the button press by moving the cursor.
    ignore_buttons: bool,
}

impl InputManager {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Read the input.
    pub fn update(&mut self) {
        let btns = self.update_buttons();
        let pad = self.update_dpad();
        self.input = if btns == Input::None { pad } else { btns }
    }

    /// Get the input read at the last update.
    #[must_use]
    pub const fn get(&self) -> Input {
        self.input
    }

    /// If true, the Select is pressed and held.
    ///
    /// This is used to animate the pressing down on UI buttons and cursor.
    #[must_use]
    pub const fn pressed(&self) -> bool {
        self.pressed
    }

    /// The vertical pixel offset for the cursor.
    ///
    /// It is used to slightly jitter the cursor when reaching the end of the item list
    /// to indicate that there are no more items to show.
    ///
    /// The `hitting_wall` arg indicate if the cursor is at the start
    /// or at the end of the list.
    #[must_use]
    pub const fn jitter(&self, hitting_wall: bool) -> i8 {
        if !hitting_wall {
            return 0;
        }
        if self.held_for < 30 || self.held_for % 30 <= 5 {
            return 0;
        }
        match self.old_dpad {
            DPad4::Left | DPad4::Up => -1,
            DPad4::Right | DPad4::Down => 1,
            DPad4::None => 0,
        }
    }

    fn update_buttons(&mut self) -> Input {
        let new_buttons = read_buttons(Peer::COMBINED);
        let released = new_buttons.just_released(&self.old_buttons);
        self.old_buttons = new_buttons;

        if self.ignore_buttons {
            if !new_buttons.any() {
                self.ignore_buttons = false;
            }
            self.pressed = false;
            return Input::None;
        }
        self.pressed = new_buttons.s || new_buttons.e;

        if released.s || released.e {
            Input::Select
        } else if released.w || released.menu {
            Input::Back
        } else {
            self.ignore_buttons = false;
            Input::None
        }
    }

    fn update_dpad(&mut self) -> Input {
        let new_dpad = read_pad(Peer::COMBINED).unwrap_or_default().as_dpad4();
        let mut dpad_pressed = new_dpad.just_pressed(self.old_dpad);

        // If a direction on the pad is held for long enough,
        // step in this direction on every frame
        // instead of only just when it's pressed.
        if new_dpad == DPad4::None {
            self.held_for = 0;
        } else {
            self.held_for += 1;
            self.ignore_buttons = true;
        }
        if self.held_for > 30 && self.held_for.is_multiple_of(4) {
            dpad_pressed = new_dpad;
        }

        self.old_dpad = new_dpad;
        match dpad_pressed {
            DPad4::None => Input::None,
            DPad4::Left => Input::Left,
            DPad4::Right => Input::Right,
            DPad4::Up => Input::Up,
            DPad4::Down => Input::Down,
        }
    }
}
