pub struct ThreadPool;

impl ThreadPool {
    pub fn new() -> Self {
        Self
    }

    pub fn execute<T: Fn()>(&self, work: T) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pool: ThreadPool = ThreadPool::new();
        pool.execute(|| println!("Hello from thead"));
    }
}
