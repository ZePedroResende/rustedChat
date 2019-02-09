extern crate tokio;

pub use self::chat::Server;
mod chat;

fn main() -> Result<(), Box<std::error::Error>>   {
    let vec = vec![
     "127.0.0.1:12347"
    ];

    let s = chat::Server::new(vec[0],vec);
    s.run()
}
