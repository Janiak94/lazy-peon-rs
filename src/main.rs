use clap::Parser;
use lazy_peon_rs::{backend, generator, keyboard, mouse};
use rand::{Rng, SeedableRng};

async fn run_async<T: FnMut() -> Result<(), Box<dyn std::error::Error>>>(mut f: T, period_ms: u64) {
    loop {
        f().unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(period_ms)).await;
    }
}

async fn run_async_keyboard<T: FnMut() -> Result<(), Box<dyn std::error::Error>>>(
    mut f: T,
    min_period_ms: u64,
    max_period_ms: u64,
) {
    let mut rng = rand::rngs::StdRng::from_entropy();
    loop {
        f().unwrap();
        let period_ms = rng.gen_range(min_period_ms..max_period_ms);
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
    let step_generator = generator::RandomWalk::new(args.step_size_px, rng);
    let mouse_backend = backend::EnigoMouseBackend::new();

    tracing::info!("Mouse backend: {}", mouse_backend);

    let mut mouse = mouse::MouseAgent::new(mouse_backend, step_generator);
    let mouse_update_freq = 1000.0 / args.updates_per_sec;

    tracing::info!("Mouse update frequency: {} ms", mouse_update_freq);
    tracing::info!("Mouse step size: {} px", args.step_size_px);

    let rng = rand::rngs::StdRng::from_entropy();
    let keyboard_backend = backend::EnigoKeyboardBackend::new();
    let key_generator = generator::RandomKeyGenerator::new(rng);
    let mut keyboard = keyboard::KeyboardAgent::new(keyboard_backend, key_generator);

    // Keyboard periods
    let min_period_ms = 100;
    let max_period_ms = 1000;

    tracing::info!("Starting loop");
    tokio::select! {
        v = run_async(move || {
            mouse.update();
            Ok(())
        }, mouse_update_freq as u64) => v,
        v = run_async_keyboard(move || {
            keyboard.update();
            Ok(())
        }, min_period_ms, max_period_ms) => v,
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
