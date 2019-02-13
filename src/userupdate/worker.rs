extern crate ratelimit;

use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

pub struct Worker {
    ratelimit_handle: Option<ratelimit::Handle>,
    thread: Option<thread::JoinHandle<()>>,
    termination_sender: mpsc::Sender<()>,
    termination_receiver: Option<mpsc::Receiver<()>>,
}

fn run(handle: &ratelimit::Handle) {}

impl Worker {
    pub fn new(ratelimit_handle: ratelimit::Handle) -> Self {
        let (termination_sender, termination_receiver) = mpsc::channel();
        Worker {
            ratelimit_handle: Some(ratelimit_handle),
            termination_sender,
            termination_receiver: Some(termination_receiver),
            thread: None,
        }
    }

    pub fn launch(mut self) -> Worker {
        let receiver = std::mem::replace(&mut self.termination_receiver, None)
            .expect("could not launch worker: termination receiver could not be unwrapped");
        let handle = std::mem::replace(&mut self.ratelimit_handle, None)
            .expect("could not launch worker: ratelimit_handle could not be unwrapped");

        self.thread = Some(thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(500));

            // check terminating
            match receiver.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }

            run(&handle);
        }));

        self
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        if let Ok(()) = self.termination_sender.send(()) {}
    }
}
