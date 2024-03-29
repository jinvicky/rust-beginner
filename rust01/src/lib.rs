use std::sync::{Arc, mpsc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    senders: mpsc::Sender<Job>,

}
type Job = Box<dyn FnOnce() + Send + 'static>;

// struct Job;


impl ThreadPool {
    // Create a new ThreadPool

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (senders, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id,
                                     Arc::clone(&receiver))
            );
            // create some threads and store them in the vector
        }

        ThreadPool { workers, senders }
    }

    pub fn excute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.senders.send(job).unwrap();
    }

}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();

            println!("Worker {} got a job; executing.", id);
            job();
        });

        Worker { id, thread }
    }
}