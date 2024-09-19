use std::{io::Write, net::TcpStream};

use clap::Parser;
use slcs_rs::{Bitmap, CommandListBuilder, Direction, PrintProcess};

#[derive(Parser)]
struct Args {
    #[arg(required = true)]
    text: String,

    #[arg(default_value = "1")]
    amount: u8,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut stream = TcpStream::connect("10.19.89.70:9100")?;

    let mut builder = &mut CommandListBuilder::new();
    builder = builder
        .set_width(360)
        .set_length(224, 24, slcs_rs::MediaType::Gap)
        .set_direction(Direction::BottomToTop)
        .set_process(PrintProcess::ThermalTransfer)
        .set_speed(3)
        .set_density(14)
        .set_cutting(true, args.amount as u32);

    let label = builder
        .bitmap(110, 14, Bitmap::from_str_as_qr(200, &args.text).unwrap())
        .print_many(args.amount as u32);

    stream.write_all(&label.clone().data())?;

    Ok(())
}
