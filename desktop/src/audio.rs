use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ferroboy::SAMPLE_RATE;

const MAX_QUEUED: usize = SAMPLE_RATE as usize / 4;

pub struct Audio {
    queue: Arc<Mutex<VecDeque<f32>>>,
    _stream: cpal::Stream,
}

impl Audio {
    pub fn new() -> Option<Self> {
        let device = cpal::default_host().default_output_device()?;
        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate: SAMPLE_RATE,
            buffer_size: cpal::BufferSize::Default,
        };

        let queue = Arc::new(Mutex::new(VecDeque::new()));
        let playing = queue.clone();

        let stream = device
            .build_output_stream(
                config,
                move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let mut queue = playing.lock().unwrap();
                    for sample in output.iter_mut() {
                        *sample = queue.pop_front().unwrap_or(0.0);
                    }
                },
                |error| eprintln!("audio: {error}"),
                None,
            )
            .ok()?;

        stream.play().ok()?;

        Some(Self {
            queue,
            _stream: stream,
        })
    }

    pub fn queue(&self, samples: &[(f32, f32)]) {
        let mut queue = self.queue.lock().unwrap();
        if queue.len() >= MAX_QUEUED {
            return;
        }

        for &(left, right) in samples {
            queue.push_back(left);
            queue.push_back(right);
        }
    }
}
