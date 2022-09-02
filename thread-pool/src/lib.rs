use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool{
    _handles: Vec<std::thread::JoinHandle<()>>,
    sender: Sender<Box<dyn Fn() + Send>>,
}

impl ThreadPool {

    pub fn new(num_threads: u8) -> Self {
        let (sender, receiver) = channel::<Box<dyn Fn() + Send>>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut _handles = vec![];
            for _ in 0..num_threads {
                let clone = receiver.clone();
                let handle = thread::spawn(move || loop {
                    let work = clone.lock().unwrap().recv().unwrap();
                    println!("Start");
                    work();
                    println!("Finish");
                });
                _handles.push(handle);
            }
       Self { _handles, sender }
    }

    pub fn execute<T: Fn() + Send + 'static>(&self, work: T ) {
        self.sender.send(Box::new(work)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::time;

    use super::*;

    #[test]
    fn it_works() {
        let pool: ThreadPool = ThreadPool::new(10);
        let foo = || thread::sleep(time::Duration::from_secs(1)) ;
        pool.execute(foo.clone());
        pool.execute(foo);
        thread::sleep(time::Duration::from_secs(10));
    }
}
