use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use std::time::Instant;

pub struct LeakyBucket {
    pub max_tokens: u32,
    pub refill_rate: u32,
    pub tokens: Arc<Mutex<u32>>,
    pub refill_interval: Duration,
    last_call: Arc<Mutex<Instant>>,
}

impl LeakyBucket {
    pub fn new(max_tokens: u32, refill_rate: u32, refill_interval: Duration) -> Self {
        let n = LeakyBucket {
            max_tokens,
            refill_rate,
            tokens: Arc::new(Mutex::new(max_tokens)),
            refill_interval,
            last_call: Arc::new(Mutex::new(Instant::now())),
        };

        return n
    }

    pub async fn acquire(&self) {
        loop {
            let mut tokens = self.tokens.lock().await;
            let mut last_call = self.last_call.lock().await;

            let elapsed = last_call.elapsed().as_secs_f64();
            let refill_amount = (elapsed * self.refill_rate as f64) as u32;

            *tokens = (*tokens + refill_amount).min(self.max_tokens);
            *last_call = Instant::now();

            if *tokens > 0 {
                *tokens -= 1;
                return
            }

            drop(tokens);
            drop(last_call);

            sleep(self.refill_interval).await;
        }
    }
}