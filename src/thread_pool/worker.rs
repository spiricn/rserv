use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;

/// Worker thread
pub struct Worker {
    /// Thread running the command loop
    pub thread: Option<JoinHandle<()>>,
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;

/// Message the worker thread can process
pub enum Message {
    /// Instructs the worker to stop the thread
    Quit,

    // Instructs the worker to execute given function
    Execute(Job),
}

impl Worker {
    /// Creates a new worker
    ///
    /// # Arguments
    ///
    /// * 'receiver' - Receiver channel used to communicate with parent thrad
    pub fn new(receiver: Arc<Mutex<std::sync::mpsc::Receiver<Message>>>) -> Worker {
        Worker {
            thread: Some(std::thread::spawn(move || loop {
                // Get a message
                let message = receiver.lock().unwrap().recv().unwrap();

                // Process it
                match message {
                    Message::Quit => break,
                    Message::Execute(job) => job(),
                }
            })),
        }
    }
}
