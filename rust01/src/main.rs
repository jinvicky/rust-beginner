use crate::http::server;

mod http {
    pub mod server;
}


fn main() {
    server::run();
}
