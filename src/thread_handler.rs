use std::{thread, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

pub struct PoolCreationError {
    message: String,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new thread pool.
    /// 
    /// the size is the number of thread pool.
    /// 
    /// # Panics
    /// 
    /// the `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        // if conditions inside assert! marco cause panic
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)).unwrap());
        }

        ThreadPool { workers, sender }
    }

    /// Build thread pool with error handling
    /// 
    /// Parameter size is number of threads that pool can hold
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0  {
            return Err(PoolCreationError{
                message: "You must have at least one thread".to_string(),
            })
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let worker = Worker::new(id, receiver.clone()).unwrap();
            workers.push(worker);
        }

        Ok(ThreadPool{ workers, sender })
    }

    pub fn execute<F>(&self, f: F) 
    where 
        F: FnOnce(),
        F: Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, std::io::Error> {
        let builder = thread::Builder::new();
        let thread = match builder.spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        }){
            Ok(thread) => thread,
            Err(e) => return Err(e),
        };
        Ok(Worker { id, thread })
    }
}