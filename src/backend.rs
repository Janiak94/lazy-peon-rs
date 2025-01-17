use super::types::PointF32;
use enigo::{Keyboard as EnigoKeyboard, Mouse as EnigoMouse};

pub trait KeyboardBackend {
    fn press_key(&mut self, key: char);
}

pub struct EnigoKeyboardBackend {
    backend: enigo::Enigo,
}

impl EnigoKeyboardBackend {
    pub fn new() -> Self {
        EnigoKeyboardBackend {
            backend: enigo::Enigo::new(&enigo::Settings::default())
                .expect("could not create enigo"),
        }
    }
}

impl Default for EnigoKeyboardBackend {
    fn default() -> Self {
        EnigoKeyboardBackend::new()
    }
}

impl KeyboardBackend for EnigoKeyboardBackend {
    fn press_key(&mut self, key: char) {
        self.backend
            .key(enigo::Key::Unicode(key), enigo::Direction::Click)
            .expect("failed to press key");
    }
}

pub trait MouseBackend {
    fn move_mouse(&mut self, pos: PointF32);

    fn read_mouse_position(&self) -> PointF32;
}

pub struct EnigoMouseBackend {
    backend: enigo::Enigo,
}

impl EnigoMouseBackend {
    pub fn new() -> Self {
        EnigoMouseBackend {
            backend: enigo::Enigo::new(&enigo::Settings::default())
                .expect("could not create enigo"),
        }
    }
}

impl Default for EnigoMouseBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl MouseBackend for EnigoMouseBackend {
    fn move_mouse(&mut self, pos: PointF32) {
        self.backend
            .move_mouse(pos.x as i32, pos.y as i32, enigo::Coordinate::Abs)
            .expect("Failed to move mouse");
    }

    fn read_mouse_position(&self) -> PointF32 {
        let (x, y) = self
            .backend
            .location()
            .expect("Failed to get mouse location");
        PointF32::new(x as f32, y as f32)
    }
}

impl core::fmt::Display for EnigoMouseBackend {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "enigo")
    }
}

struct MouseRsMouseBackend {
    backend: mouse_rs::Mouse,
}

impl MouseRsMouseBackend {
    #[allow(unused)]
    pub fn new() -> Self {
        MouseRsMouseBackend {
            backend: mouse_rs::Mouse::new(),
        }
    }
}

impl MouseBackend for MouseRsMouseBackend {
    fn move_mouse(&mut self, pos: PointF32) {
        self.backend
            .move_to(pos.x as i32, pos.y as i32)
            .expect("failed to move mouse");
    }

    fn read_mouse_position(&self) -> PointF32 {
        let pos = self
            .backend
            .get_position()
            .expect("could not get mouse position");
        PointF32::new(pos.x as f32, pos.y as f32)
    }
}

impl core::fmt::Display for MouseRsMouseBackend {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "mouse_rs")
    }
}
