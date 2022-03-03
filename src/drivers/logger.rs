pub struct Logger;

pub trait Log {
    fn info(&self, msg: String);
}

impl Log for Logger {
    fn info(&self, msg: String) {
        println!("{}", msg);
    }
}

impl Logger {
    pub fn new() -> Logger {
        return Logger {};
    }
}
