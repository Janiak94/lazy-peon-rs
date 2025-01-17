use crate::backend::KeyboardBackend;
use crate::generator::KeyGenerator;

pub struct KeyboardAgent<B, K>
where
    B: KeyboardBackend,
    K: KeyGenerator,
{
    backend: B,
    key_generator: K,
}

impl<B, K> KeyboardAgent<B, K>
where
    B: KeyboardBackend,
    K: KeyGenerator,
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
