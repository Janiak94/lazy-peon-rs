use crate::backend::MouseBackend;
use crate::generator::StepGenerator;
use crate::types::PointF32;

pub struct MouseAgent<B, S>
where
    B: MouseBackend,
{
    backend: B,
    last_read_pos: PointF32,
    pos: PointF32,
    step_generator: S,
}

impl<B, S> MouseAgent<B, S>
where
    B: MouseBackend,
    S: StepGenerator,
{
    pub fn new(backend: B, step_generator: S) -> Self {
        let pos = backend.read_mouse_position();
        MouseAgent {
            backend,
            last_read_pos: pos,
            pos,
            step_generator,
        }
    }

    pub fn update(&mut self) {
        // Check if mouse has manually moved since last update.
        let current_pos = self.backend.read_mouse_position();
        let delta = current_pos - self.last_read_pos;
        if delta.x.abs() > 1.0 || delta.y.abs() > 1.0 {
            // If mouse has moved, self.pos is invalid.
            self.pos = current_pos;
            tracing::debug!("Manual movement detected. Resetting position.");
        }
        self.last_read_pos = current_pos;

        // Update float position.
        let direction = self.step_generator.step();
        self.pos += direction;

        // If the delta is greater than 1 pixel, move the mouse.
        let delta = self.pos - self.last_read_pos;
        if delta.x.abs() > 1.0 || delta.y.abs() > 1.0 {
            self.backend.move_mouse(self.pos);
            tracing::debug!("Moving mouse to: {}", self.pos);
        }

        self.last_read_pos = self.backend.read_mouse_position();
    }
}
