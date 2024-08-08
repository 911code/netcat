use clap::{Parser, Subcommand};
use std::net::IpAddr;

type IOResult<T> = std::io::Result<T>;

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_ADDRESS: &str = "127.0.0.1";


#[derive(Parser, Debug)]
struct CLIArg {
#[clap(long, short, default_value = DEFAULT_ADDRESS, global = true)]
pub address: IpAddr,

#[clap(long, short, default_value_t = DEFAULT_PORT, global = true)]
port: u16,

#[clap(long, short = 'd', global = true)]
log_disable: bool,

#[clap(subcommand)]
subcommand: CLISubcommand,
}

#[derive(Subcommand, Debug)]
enum CLISubcommand {Server, Client}

fn main() {
    println!("Welcome to Basic Netcat Server/Client CLI");

    let cli_arg = CLIArg::parse();
    let (address, port) = (cli_arg.address, cli_arg.port);
    let socket_address = format!("{}:{}", address, port);
    println!("The socket_IpAddress:  {}", socket_address);

    match match cli_arg.subcommand {
        CLISubcommand::Server => start_server(socket_address),
        CLISubcommand::Client => start_client(socket_address),
    } {
        Ok(_) => println!(" \nProgram exited successfully"),
        Err(error) => println!(" \nProgram exited with an error: {}", error),
    }
}


fn start_client(socket_address: String) -> IOResult<()>{
        print!("Start client connection @ {}", socket_address);
        Ok(())
}
fn start_server(socket_address: String)  -> IOResult<()>{
    print!("Start server connection @ {}", socket_address);
        Ok(())
}