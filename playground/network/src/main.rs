
mod single_thread_server;

fn main() {
    let listener = match single_thread_server::check_connection("127.0.0.1:8080".to_owned()) {
        Ok(listener) => listener,
        Err(err) => panic!("{}", err)
    };
}
