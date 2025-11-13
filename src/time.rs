use rand::{Rng, RngCore};
pub use tokio::time::interval;
pub use tokio::time::Duration;
use tokio::time::Instant;

pub struct RandomInterval {
    min: f32,
    max: f32,
    inner: tokio::time::Interval,
    rng: Box<dyn RngCore>,
}

fn random_duration<Rng>(min: f32, max: f32, rng: &mut Rng) -> Duration
where
    Rng: rand::RngCore,
{
    Duration::from_secs_f32(rng.gen_range(min..max))
}

impl RandomInterval {
    fn new(min: f32, max: f32) -> RandomInterval {
        let mut rng = rand::thread_rng();
        let dur = random_duration(min, max, &mut rng);
        let inner = interval(dur);
        Self {
            min,
            max,
            inner,
            rng: Box::new(rng),
        }
    }

    pub async fn tick(&mut self) -> Instant {
        let tick = self.inner.tick().await;
        self.inner = interval(random_duration(self.min, self.max, &mut self.rng));
        // Consume first tick
        self.inner.tick().await;
        tick
    }
}

pub fn random_interval(min: f32, max: f32) -> RandomInterval {
    RandomInterval::new(min, max)
}
