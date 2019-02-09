extern crate tokio;


pub use self::chat::Server;

mod chat{

    use std::net::SocketAddr;
    use std::vec::Vec;
    use tokio::prelude::*;
    use tokio::io::copy;
    use tokio::net::{TcpListener, TcpStream};

    #[derive(Debug)]
    pub struct Server{
        socket_addr: SocketAddr,
        //listener: TcpListener,
        addresses: Vec<SocketAddr>,
    }

    impl Server {

        pub fn new(my_address: &str, addresses: Vec<&str> ) -> Server{
            let addr = string_to_addres(my_address);
            /*
            let list = match TcpListener::bind(&addr){
                Ok(result) => result,
                Err(result) => panic!("Failed to bind to socket {}", result),
            };
            */
            let out_addr= addresses.iter().map(|x| string_to_addres(x)).collect();

            Server {
                socket_addr: addr,
                //listener: list,
                addresses: out_addr,
            }
        }

        pub fn run(&self) -> Result<(), Box<std::error::Error>>  {
            let listener = TcpListener::bind(&self.socket_addr)?;
            let server = listener.incoming()
                .for_each( move |socket| {
                    handler(socket); 
                    Ok(())
                })
            .map_err(|e| eprintln!("accept failed = {:?}", e));

            tokio::run(server);
            Ok(())
        }

    }

    fn string_to_addres(addr : &str) -> SocketAddr {
        match addr.to_string().parse::<SocketAddr>(){
            Ok(result) => result,
            Err(result) => panic!("Failed to parse address {}", result),
        }       
    }

        fn handler( socket: TcpStream){
            let (reader, writer) = socket.split();
            let bytes_copied = copy(reader, writer);
            let handle_conn = bytes_copied.map(|amt| {
                println!("wrote {:?} bytes", amt)
            }).map_err(|err| {
                eprintln!("IO error {:?}", err)
            });

            tokio::spawn(handle_conn);
        }

}
