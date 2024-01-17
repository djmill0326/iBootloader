pub mod haptloader;

fn main() -> Result<(), ()> {
    println!("[hapt-server] ehpt");
    haptloader::init().map(|_| eprintln!("wtf dood"))
}