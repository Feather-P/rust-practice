use std::{
    sync::{
        mpsc::{self, channel, Receiver}, Arc, Mutex
    }, thread
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// 创建一个新的线程池。
    ///
    /// size 是池中线程的数量。
    ///
    /// # Panics
    ///
    /// 如果 size 为 0，`new` 方法会 panic。
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // 在不满足条件时其会panic! 
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            println!("Shutting down worker: {}", worker.id);
            worker.thread_handler.join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread_handler: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, recv: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread_handler = thread::spawn(move || {
            loop {
                let message = recv.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        println!("worker {} received a job, excuting now.", id);
                        job();
                    }
                    Err(_) => {
                        println!("Channel disconnected, shutting down thread");
                        break;
                    }
                }
            }
        });
        Worker { id, thread_handler }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
