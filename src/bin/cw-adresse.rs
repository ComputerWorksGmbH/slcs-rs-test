use slcs_rs::*;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("10.19.89.70:9100")?;

    let mut builder = &mut CommandListBuilder::new();
    builder = builder
        .set_width(360)
        .set_length(224, 24, slcs_rs::MediaType::Gap)
        .set_direction(Direction::BottomToTop)
        .set_process(PrintProcess::ThermalTransfer)
        .set_speed(3)
        .set_density(14)
        .set_cutting(true, 5);

    let label = builder
        .text_ex(25, 50, true, Rotation::None, '1', "ComputerWorks GmbH")
        .text_ex(25, 80, true, Rotation::None, '1', "Schwarzwaldstr. 67")
        .text_ex(25, 110, true, Rotation::None, '1', "79539 Loerrach")
        .text_ex(25, 160, true, Rotation::None, '1', "Tel: +49 7621 40180")
        .print_many(5);

    stream.write_all(&label.clone().data())?;

    Ok(())
}
