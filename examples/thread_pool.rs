use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if let Some(sender) = &self.sender {
            let job = Box::new(f);
            sender.send(job).expect("ThreadPool: send job failed");
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Drop the sender so workers will get Err on recv and exit
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                handle.join().expect("Worker thread panicked");
            }
        }
    }
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    // run the job
                    //println!("Worker {} got a job; executing.", id);
                    job();
                }
                Err(_) => {
                    // Channel closed, exit the loop
                    //println!("Worker {} disconnecting; shutting down.", id);
                    break;
                }
            }
        });

        Worker { id, handle: Some(handle) }
    }
}

fn main() {
    println!("-- 简单线程池示例 --");

    let pool = ThreadPool::new(4);

    for i in 0..8 {
        pool.execute(move || {
            println!("task {} started", i);
            std::thread::sleep(std::time::Duration::from_millis(300));
            println!("task {} finished", i);
        });
    }

    println!("已提交任务，主线程将在 ThreadPool 被 drop 时等待所有任务完成（end of main）");
}
