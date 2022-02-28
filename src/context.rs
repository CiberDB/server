use openssl::{rsa::Rsa, pkey::Private};

pub struct Settings {
    pub host: String,
    pub port: String,
    pub rsa: Rsa<Private>,
}

pub struct Context {
    pub settings: Settings,
}