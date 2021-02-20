use super::worker::Message;
use super::worker::Worker;
use std::sync::Arc;
use std::sync::Mutex;

/// Thread pool
pub struct ThreadPool {
    /// List of worker threads
    workers: Vec<Worker>,

    /// Channel used to send messages to one of the threads
    sender: std::sync::mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Creates a ThreadPool
    ///
    /// # Arguments
    ///
    /// * 'size' - Number of worker threads
    pub fn new(size: i32) -> ThreadPool {
        assert!(size > 0);

        // Create a channel trough which we'll send jobs to be executed (single sender, multiple receivers)
        let (sender, receiver) = std::sync::mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        // Spin up workers
        let mut workers = Vec::new();
        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        ThreadPool {
            workers: workers,
            sender,
        }
    }

    ///
    /// Execute arbitrary code on first available worker thread
    ///
    /// # Arguments
    ///
    /// * 'fnc' - Function to execute
    pub fn execute<F>(&self, fnc: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Message::Execute(Box::new(fnc))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Send a quit message for each of the workers
        for _ in &self.workers {
            self.sender.send(Message::Quit).unwrap();
        }

        // Wait for threads to finish
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
mod test {
    #[test]
    fn execute() {
        let tp = super::ThreadPool::new(1);

        for _ in 0..10 {
            tp.execute(|| {});
        }
    }
}
