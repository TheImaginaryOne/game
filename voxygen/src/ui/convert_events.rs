use conrod::{
    event::Input,
    input::{self, Key},
    Scalar,
};

use glutin::{
    dpi::{LogicalPosition, LogicalSize},
    ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta, Touch, TouchPhase, VirtualKeyCode, WindowEvent,
};

pub fn convert(event: Event, win_w: f64, win_h: f64) -> Option<Input> {
    let dpi_factor = 1.0;

    let tx = |x: Scalar| (x / dpi_factor) - win_w / 2.0;
    let ty = |y: Scalar| -((y / dpi_factor) - win_h / 2.0);

    let event = match event {
        Event::WindowEvent { event, .. } => event,
        _ => return None,
    };

    match event {
        WindowEvent::Resized(LogicalSize { width, height }) => {
            let w = (width as Scalar / dpi_factor) as u32;
            let h = (height as Scalar / dpi_factor) as u32;
            Some(Input::Resize(w, h).into())
        },
        WindowEvent::ReceivedCharacter(ch) => {
            let string = match ch {
                // Ignore control characters and return ascii for Text event (like sdl2).
                '\u{7f}' | // Delete
                '\u{1b}' | // Escape
                '\u{8}'  | // Backspace
                '\r' | '\n' | '\t' => "".to_string(),
                _ => ch.to_string()
            };
            Some(Input::Text(string).into())
        },
        WindowEvent::Focused(focused) => Some(Input::Focus(focused).into()),
        WindowEvent::KeyboardInput { input, .. } => input.virtual_keycode.map(|key| match input.state {
            ElementState::Pressed => Input::Press(input::Button::Keyboard(map_key(key))).into(),
            ElementState::Released => Input::Release(input::Button::Keyboard(map_key(key))).into(),
        }),
        WindowEvent::Touch(Touch {
            phase,
            location: LogicalPosition { x, y },
            id,
            ..
        }) => {
            let phase = match phase {
                TouchPhase::Started => input::touch::Phase::Start,
                TouchPhase::Moved => input::touch::Phase::Move,
                TouchPhase::Cancelled => input::touch::Phase::Cancel,
                TouchPhase::Ended => input::touch::Phase::End,
            };
            let xy = [tx(x), ty(y)];
            let id = input::touch::Id::new(id);
            let touch = input::Touch {
                phase: phase,
                id: id,
                xy: xy,
            };
            Some(Input::Touch(touch).into())
        },

        WindowEvent::CursorMoved {
            position: LogicalPosition { x, y },
            ..
        } => {
            let x = tx(x as Scalar);
            let y = ty(y as Scalar);
            let motion = input::Motion::MouseCursor { x: x, y: y };
            Some(Input::Motion(motion).into())
        },
        WindowEvent::MouseWheel { delta, .. } => match delta {
            MouseScrollDelta::PixelDelta(LogicalPosition { x, y }) => {
                let x = x as Scalar / dpi_factor;
                let y = -y as Scalar / dpi_factor;
                let motion = input::Motion::Scroll { x: x, y: y };
                Some(Input::Motion(motion).into())
            },

            MouseScrollDelta::LineDelta(x, y) => {
                // This should be configurable (we should provide a LineDelta event to allow for this).
                const ARBITRARY_POINTS_PER_LINE_FACTOR: Scalar = 10.0;
                let x = ARBITRARY_POINTS_PER_LINE_FACTOR * x as Scalar;
                let y = ARBITRARY_POINTS_PER_LINE_FACTOR * -y as Scalar;
                Some(Input::Motion(input::Motion::Scroll { x: x, y: y }).into())
            },
        },
        WindowEvent::MouseInput { state, button, .. } => match state {
            ElementState::Pressed => Some(Input::Press(input::Button::Mouse(map_mouse(button))).into()),
            ElementState::Released => Some(Input::Release(input::Button::Mouse(map_mouse(button))).into()),
        },
        WindowEvent::Refresh => Some(Input::Redraw),
        _ => None,
    }
}

/// Maps winit's key to a conrod `Key`.
pub fn map_key(keycode: VirtualKeyCode) -> input::keyboard::Key {
    match keycode {
        VirtualKeyCode::Key0 => Key::D0,
        VirtualKeyCode::Key1 => Key::D1,
        VirtualKeyCode::Key2 => Key::D2,
        VirtualKeyCode::Key3 => Key::D3,
        VirtualKeyCode::Key4 => Key::D4,
        VirtualKeyCode::Key5 => Key::D5,
        VirtualKeyCode::Key6 => Key::D6,
        VirtualKeyCode::Key7 => Key::D7,
        VirtualKeyCode::Key8 => Key::D8,
        VirtualKeyCode::Key9 => Key::D9,
        VirtualKeyCode::A => Key::A,
        VirtualKeyCode::B => Key::B,
        VirtualKeyCode::C => Key::C,
        VirtualKeyCode::D => Key::D,
        VirtualKeyCode::E => Key::E,
        VirtualKeyCode::F => Key::F,
        VirtualKeyCode::G => Key::G,
        VirtualKeyCode::H => Key::H,
        VirtualKeyCode::I => Key::I,
        VirtualKeyCode::J => Key::J,
        VirtualKeyCode::K => Key::K,
        VirtualKeyCode::L => Key::L,
        VirtualKeyCode::M => Key::M,
        VirtualKeyCode::N => Key::N,
        VirtualKeyCode::O => Key::O,
        VirtualKeyCode::P => Key::P,
        VirtualKeyCode::Q => Key::Q,
        VirtualKeyCode::R => Key::R,
        VirtualKeyCode::S => Key::S,
        VirtualKeyCode::T => Key::T,
        VirtualKeyCode::U => Key::U,
        VirtualKeyCode::V => Key::V,
        VirtualKeyCode::W => Key::W,
        VirtualKeyCode::X => Key::X,
        VirtualKeyCode::Y => Key::Y,
        VirtualKeyCode::Z => Key::Z,
        VirtualKeyCode::Apostrophe => Key::Unknown,
        VirtualKeyCode::Backslash => Key::Backslash,
        VirtualKeyCode::Back => Key::Backspace,
        // K::CapsLock => Key::CapsLock,
        VirtualKeyCode::Delete => Key::Delete,
        VirtualKeyCode::Comma => Key::Comma,
        VirtualKeyCode::Down => Key::Down,
        VirtualKeyCode::End => Key::End,
        VirtualKeyCode::Return => Key::Return,
        VirtualKeyCode::Equals => Key::Equals,
        VirtualKeyCode::Escape => Key::Escape,
        VirtualKeyCode::F1 => Key::F1,
        VirtualKeyCode::F2 => Key::F2,
        VirtualKeyCode::F3 => Key::F3,
        VirtualKeyCode::F4 => Key::F4,
        VirtualKeyCode::F5 => Key::F5,
        VirtualKeyCode::F6 => Key::F6,
        VirtualKeyCode::F7 => Key::F7,
        VirtualKeyCode::F8 => Key::F8,
        VirtualKeyCode::F9 => Key::F9,
        VirtualKeyCode::F10 => Key::F10,
        VirtualKeyCode::F11 => Key::F11,
        VirtualKeyCode::F12 => Key::F12,
        VirtualKeyCode::F13 => Key::F13,
        VirtualKeyCode::F14 => Key::F14,
        VirtualKeyCode::F15 => Key::F15,
        // K::F16 => Key::F16,
        // K::F17 => Key::F17,
        // K::F18 => Key::F18,
        // K::F19 => Key::F19,
        // K::F20 => Key::F20,
        // K::F21 => Key::F21,
        // K::F22 => Key::F22,
        // K::F23 => Key::F23,
        // K::F24 => Key::F24,
        // Possibly next code.
        // K::F25 => Key::Unknown,
        VirtualKeyCode::Numpad0 => Key::NumPad0,
        VirtualKeyCode::Numpad1 => Key::NumPad1,
        VirtualKeyCode::Numpad2 => Key::NumPad2,
        VirtualKeyCode::Numpad3 => Key::NumPad3,
        VirtualKeyCode::Numpad4 => Key::NumPad4,
        VirtualKeyCode::Numpad5 => Key::NumPad5,
        VirtualKeyCode::Numpad6 => Key::NumPad6,
        VirtualKeyCode::Numpad7 => Key::NumPad7,
        VirtualKeyCode::Numpad8 => Key::NumPad8,
        VirtualKeyCode::Numpad9 => Key::NumPad9,
        VirtualKeyCode::NumpadComma => Key::NumPadDecimal,
        VirtualKeyCode::Divide => Key::NumPadDivide,
        VirtualKeyCode::Multiply => Key::NumPadMultiply,
        VirtualKeyCode::Subtract => Key::NumPadMinus,
        VirtualKeyCode::Add => Key::NumPadPlus,
        VirtualKeyCode::NumpadEnter => Key::NumPadEnter,
        VirtualKeyCode::NumpadEquals => Key::NumPadEquals,
        VirtualKeyCode::LShift => Key::LShift,
        VirtualKeyCode::LControl => Key::LCtrl,
        VirtualKeyCode::LAlt => Key::LAlt,
        //        VirtualKeyCode::LMenu => Key::LGui,
        VirtualKeyCode::RShift => Key::RShift,
        VirtualKeyCode::RControl => Key::RCtrl,
        VirtualKeyCode::RAlt => Key::RAlt,
        //        VirtualKeyCode::RMenu => Key::RGui,
        // Map to backslash?
        // K::GraveAccent => Key::Unknown,
        VirtualKeyCode::Home => Key::Home,
        VirtualKeyCode::Insert => Key::Insert,
        VirtualKeyCode::Left => Key::Left,
        VirtualKeyCode::LBracket => Key::LeftBracket,
        // K::Menu => Key::Menu,
        VirtualKeyCode::Minus => Key::Minus,
        VirtualKeyCode::Numlock => Key::NumLockClear,
        VirtualKeyCode::PageDown => Key::PageDown,
        VirtualKeyCode::PageUp => Key::PageUp,
        VirtualKeyCode::Pause => Key::Pause,
        VirtualKeyCode::Period => Key::Period,
        // K::PrintScreen => Key::PrintScreen,
        VirtualKeyCode::Right => Key::Right,
        VirtualKeyCode::RBracket => Key::RightBracket,
        // K::ScrollLock => Key::ScrollLock,
        VirtualKeyCode::Semicolon => Key::Semicolon,
        VirtualKeyCode::Slash => Key::Slash,
        VirtualKeyCode::Space => Key::Space,
        VirtualKeyCode::Tab => Key::Tab,
        VirtualKeyCode::Up => Key::Up,
        // K::World1 => Key::Unknown,
        // K::World2 => Key::Unknown,
        _ => Key::Unknown,
    }
}

/// Maps winit's mouse button to conrod's mouse button.
pub fn map_mouse(mouse_button: MouseButton) -> input::MouseButton {
    match mouse_button {
        MouseButton::Left => input::MouseButton::Left,
        MouseButton::Right => input::MouseButton::Right,
        MouseButton::Middle => input::MouseButton::Middle,
        MouseButton::Other(0) => input::MouseButton::X1,
        MouseButton::Other(1) => input::MouseButton::X2,
        MouseButton::Other(2) => input::MouseButton::Button6,
        MouseButton::Other(3) => input::MouseButton::Button7,
        MouseButton::Other(4) => input::MouseButton::Button8,
        _ => input::MouseButton::Unknown,
    }
}
