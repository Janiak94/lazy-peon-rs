use clap::Parser;
use nalgebra as na;
use rand::SeedableRng;

mod mouse {
    use crate::na;
    use enigo::Mouse as EnigoMouse;

    pub(crate) trait MouseBackend {
        fn move_mouse(&mut self, pos: na::Point2<f32>);

        fn read_mouse_position(&self) -> na::Point2<f32>;
    }

    pub(crate) struct EnigoMouseBackend {
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

    impl MouseBackend for EnigoMouseBackend {
        fn move_mouse(&mut self, pos: na::Point2<f32>) {
            self.backend
                .move_mouse(pos.x as i32, pos.y as i32, enigo::Coordinate::Abs)
                .expect("Failed to move mouse");
        }

        fn read_mouse_position(&self) -> na::Point2<f32> {
            let (x, y) = self
                .backend
                .location()
                .expect("Failed to get mouse location");
            na::Point2::new(x as f32, y as f32)
        }
    }

    impl core::fmt::Display for EnigoMouseBackend {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "enigo")
        }
    }

    #[allow(unused)]
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
        fn move_mouse(&mut self, pos: na::Point2<f32>) {
            self.backend
                .move_to(pos.x as i32, pos.y as i32)
                .expect("failed to move mouse");
        }

        fn read_mouse_position(&self) -> na::Point2<f32> {
            let pos = self
                .backend
                .get_position()
                .expect("could not get mouse position");
            na::Point2::new(pos.x as f32, pos.y as f32)
        }
    }

    impl core::fmt::Display for MouseRsMouseBackend {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "mouse_rs")
        }
    }

    pub(crate) struct RngMouse<B, S>
    where
        B: crate::mouse::MouseBackend,
    {
        backend: B,
        last_read_pos: na::Point2<f32>,
        pos: na::Point2<f32>,
        step_generator: S,
    }

    impl<B, S> RngMouse<B, S>
    where
        B: MouseBackend,
        S: crate::random::StepGenerator,
    {
        pub fn new(backend: B, step_generator: S) -> Self {
            let pos = backend.read_mouse_position();
            RngMouse {
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
}

mod keyboard {
    use enigo::Keyboard as EnigoKeyboard;

    pub struct RngKeyboard<B, K>
    where
        B: KeyboardBackend,
        K: crate::random::KeyGenerator,
    {
        backend: B,
        key_generator: K,
    }

    impl<B, K> RngKeyboard<B, K>
    where
        B: KeyboardBackend,
        K: crate::random::KeyGenerator,
    {
        pub fn new(backend: B, key_generator: K) -> Self {
            Self {
                backend,
                key_generator,
            }
        }

        pub fn update(&mut self) {
            self.backend.press_key(self.key_generator.next_key());
        }
    }

    pub(crate) trait KeyboardBackend {
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

    impl KeyboardBackend for EnigoKeyboardBackend {
        fn press_key(&mut self, key: char) {
            self.backend
                .key(enigo::Key::Unicode(key), enigo::Direction::Click)
                .expect("failed to press key");
        }
    }
}

mod random {
    use super::na;
    use rand::{Rng, SeedableRng};

    pub(crate) trait StepGenerator {
        fn step(&mut self) -> na::Vector2<f32>;
    }

    pub(crate) trait KeyGenerator {
        fn next_key(&mut self) -> char;
    }

    pub(crate) struct RandomWalk<R> {
        step_size_px: f32,
        rng: R,
    }

    impl<R: rand::RngCore> RandomWalk<R> {
        pub fn new(step_size_px: f32, rng: R) -> Self {
            RandomWalk { step_size_px, rng }
        }
    }

    impl Default for RandomWalk<rand::rngs::StdRng> {
        fn default() -> Self {
            RandomWalk::new(1.0, rand::rngs::StdRng::from_entropy())
        }
    }

    impl<R> StepGenerator for RandomWalk<R>
    where
        R: rand::RngCore,
    {
        fn step(&mut self) -> na::Vector2<f32> {
            match self.rng.gen_range(0..4) {
                0 => na::Vector2::new(0.0, self.step_size_px),
                1 => na::Vector2::new(0.0, -self.step_size_px),
                2 => na::Vector2::new(self.step_size_px, 0.0),
                3 => na::Vector2::new(-self.step_size_px, 0.0),
                _ => unreachable!(),
            }
        }
    }

    pub(crate) struct RandomKeyGenerator<R> {
        rng: R,
    }

    impl Default for RandomKeyGenerator<rand::rngs::StdRng> {
        fn default() -> Self {
            RandomKeyGenerator::new(rand::rngs::StdRng::from_entropy())
        }
    }

    impl<R> RandomKeyGenerator<R>
    where
        R: rand::RngCore,
    {
        pub fn new(rng: R) -> Self {
            RandomKeyGenerator { rng }
        }
    }

    impl<R> KeyGenerator for RandomKeyGenerator<R>
    where
        R: rand::RngCore,
    {
        fn next_key(&mut self) -> char {
            self.rng.gen_range('a'..='z')
        }
    }
}

async fn run_async<T: FnMut() -> Result<(), Box<dyn std::error::Error>>>(mut f: T, period_ms: u64) {
    loop {
        f().unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(period_ms)).await;
    }
}

#[derive(Parser, Debug)]
#[command(name = "lazy-peon-rs", about = "Lazy peon")]
struct Args {
    #[arg(short, long, default_value_t = 1.0)]
    step_size_px: f32,

    #[arg(short, long, default_value_t = 60.0)]
    updates_per_sec: f32,

    #[arg(short, long, default_value_t = 5.0)]
    keyboard_input_period: f32,

    #[arg(short, long, value_enum, default_value_t = MouseBackendType::Enigo)]
    mouse_backend: MouseBackendType,

    #[arg(short, long, value_enum, default_value_t = LogLevel::Info)]
    log_level: LogLevel,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
enum MouseBackendType {
    Enigo,
    MouseRs,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
enum LogLevel {
    Debug,
    Info,
}

impl From<LogLevel> for tracing::Level {
    fn from(log_level: LogLevel) -> Self {
        match log_level {
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let log_level: tracing::Level = args.log_level.into();
    let subscriber = tracing_subscriber::fmt().with_max_level(log_level).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let rng = rand::rngs::StdRng::from_entropy();
    let step_generator = random::RandomWalk::new(args.step_size_px, rng);
    let mouse_backend = mouse::EnigoMouseBackend::new();

    tracing::info!("Mouse backend: {}", mouse_backend);

    let mut mouse = mouse::RngMouse::new(mouse_backend, step_generator);
    let mouse_update_freq = 1000.0 / args.updates_per_sec;

    tracing::info!("Mouse update frequency: {} ms", mouse_update_freq);
    tracing::info!("Mouse step size: {} px", args.step_size_px);

    let rng = rand::rngs::StdRng::from_entropy();
    let keyboard_backend = keyboard::EnigoKeyboardBackend::new();
    let key_generator = random::RandomKeyGenerator::new(rng);
    let mut keyboard = keyboard::RngKeyboard::new(keyboard_backend, key_generator);

    tracing::info!("Starting loop");
    tokio::select! {
        v = run_async(move || {
            mouse.update();
            Ok(())
        }, mouse_update_freq as u64) => v,
        v = run_async(move || {
            keyboard.update();
            Ok(())
        }, 1000 * args.keyboard_input_period as u64) => v,
    }
}

#[cfg(test)]
mod tests {

    mod rng_mouse {

        mod with_enigo_backend {

            #[test]
            fn update() {
                // TODO: Actual tests.
                todo!();
            }
        }

        mod with_mouse_rs_backend {

            #[test]
            fn update() {
                // TODO: Actual tests.
                todo!();
            }
        }
    }
}
