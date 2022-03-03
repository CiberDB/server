pub mod handler {
    use crate::drivers::Context;

    pub fn create(_context: &Context) -> impl Fn(Vec<u8>) -> Vec<u8> {
        let handler = |_query: Vec<u8>| -> Vec<u8> {
            return vec![];
        };
        return handler;
    }
}
