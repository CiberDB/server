pub(crate) mod logger;
pub(crate) mod tcp;

use self::{logger::Logger, tcp::TCP};
use openssl::{pkey::Private, rsa::Rsa};

pub struct Settings {
    pub host: String,
    pub port: String,
    pub rsa: Rsa<Private>,
}

pub struct Context {
    pub settings: Settings,
    pub server: TCP,
    pub logger: Logger,
}

impl Context {
    pub fn new(settings: Settings) -> Context {
        let server = TCP::new(&settings);
        let logger = Logger::new();
        return Context {
            settings: settings,
            server,
            logger,
        };
    }
}
