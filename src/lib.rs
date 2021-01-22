pub struct ThreadPool;

impl ThreadPool {
  /// 新しいThreadPoolを生成する
  /// sizeがプールのスレッド数です
  /// # パニック
  /// sizeが0なら、`new`関数はパニックします
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    ThreadPool
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
  }
}
