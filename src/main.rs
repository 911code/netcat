use clap::{Parser, Subcommand};
use r3bl_ansi_color::SgrCode;
use r3bl_tui::ColorWheel;
use std::thread;
use std::{
    io::{stdin, BufRead, BufReader, BufWriter, Write},
    net::{IpAddr, TcpListener, TcpStream},
};

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
        print!("\nStart client connection @ {} ", socket_address);
        let tcp_stream = TcpStream::connect(socket_address)?;
        let (mut reader, mut writer) = (BufReader::new(&tcp_stream), BufWriter::new(&tcp_stream));
    
        // Client loop.
        loop {
            // Read user input.
            let outgoing = {
                let mut it = String::new();
                let _ = stdin().read_line(&mut it)?;
                it.as_bytes().to_vec()
            };
    
            // Exit if EOF (Ctrl+D pressed).
            if outgoing.len() == 0 {
                break;
            }
    
            // Tx user input to writer.
            let _ = writer.write(&outgoing)?;
            writer.flush()?;
    
            // Rx response from reader.
            let incoming = {
                let mut it = vec![];
                let _ = reader.read_until(b'\n', &mut it);
                it
            };
    
            // Check for EOF, and exit.
            incoming.len() == 0  && break;
    
            let display_msg = String::from_utf8_lossy(&incoming);
            let display_msg = display_msg.trim();
    
            let reset = SgrCode::Reset.to_string();
            let display_msg = format!("{}{}", display_msg, reset);
            println!(" \n{}", display_msg);
    
            // Print debug.
            let tx = format!(
                "-> Tx: '{}', size: {} bytes{}",
                String::from_utf8_lossy(&outgoing).trim(),
                outgoing.len(),
                reset,
            );
            println!("\n <- Tx: {}", tx);

            let rx = format!(
                "<- Rx: '{}', size: {} bytes{}",
                String::from_utf8_lossy(&incoming).trim(),
                incoming.len(),
                reset,
            );
            println!("\n <- Rx: {}", rx);

        }
    
        Ok(())
}
fn start_server(socket_address: String)  -> IOResult<()>{
    
    let tcp_listener = TcpListener::bind(socket_address)?;
    
        loop {
            let (tcp_stream, ..) = tcp_listener.accept()?; // This is a blocking call.

            thread::spawn(|| match handle_connection(tcp_stream) {
                Ok(_) => println!("Successfully closed connection to client..."),
                Err(_) => println!("Problem with client connection...")
            });
        }
    fn handle_connection(tcp_stream: TcpStream) -> IOResult<()> {
        println!("Start handle connection");

        let reader = &mut BufReader::new(&tcp_stream);
        let write = &mut BufWriter::new(&tcp_stream);

        loop {
            let mut incoming: Vec<u8> = vec![];

            // Read from reader.
            let num_bytes_read = reader.read_until(b'\n', &mut incoming)?;

            // Check for EOF. The stream is closed.
            num_bytes_read == 0 && break;

            // Process.
            let outgoing = process(&incoming);

            // Write to writer.
            write.write(&outgoing)?;
            let _ = write.flush()?;

            // Print debug.
            let rx = format!(
                "-> Rx(string): '{}', size: {} bytes",
                String::from_utf8_lossy(&incoming).trim(),
                incoming.len(),
            );
            println!("-> Rx(incoming) : {}", rx);

            let tx = format!(
                "<- Tx(string): '{}', size: {} bytes",
                String::from_utf8_lossy(&outgoing).trim(),
                outgoing.len()
            );
            println!("-> Tx(outgoing) : {}", tx);

        }
        println!("End handle connection - connection closed");

        Ok(())
    }

    fn process(incoming: &Vec<u8>) -> Vec<u8> {
        // Convert incoming to String, and remove any trailing whitespace (includes newline).
        let incoming = String::from_utf8_lossy(incoming);
        let incoming = incoming.trim();

        // Prepare outgoing payload.
        let outgoing = incoming.to_string();

        // Colorize it w/ a gradient.
        let outgoing = ColorWheel::lolcat_into_string(&outgoing);

        // Generate outgoing response. Add newline to the end of output (so client can process it).
        let outgoing = format!("{}\n", outgoing);

        // Return outgoing payload.
        outgoing.as_bytes().to_vec()
    }
}