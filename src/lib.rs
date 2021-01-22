use std::thread;

pub struct ThreadPool {
  workers: Vec<Worker>,
}

impl ThreadPool {
  /// 新しいThreadPoolを生成する
  /// sizeがプールのスレッド数です
  /// # パニック
  /// sizeが0なら、`new`関数はパニックします
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      let worker = Worker::new(id);
      workers.push(worker);
    }

    ThreadPool { workers }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
  }
}

struct Worker {
  id: usize,
  handle: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize) -> Worker {
    let thread = thread::spawn(|| {});

    Worker {
      id: id,
      handle: thread,
    }
  }
}
