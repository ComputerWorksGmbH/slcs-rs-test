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
        .set_cutting(true, 2);

    let label = builder
        .text_ex(15, 180, true, Rotation::D270, '5', "HWDB")
        .text_ex(85, 185, true, Rotation::D270, 'n', "19.07.2024")
        .blockdraw(150, 24, 155, 190, BlockOpts::Overwrite)
        .bitmap(
            160,
            14,
            Bitmap::from_str_as_qr(200, "#hw_1E1DEA8B-AE27-40E6-8051-0901C48463E4").unwrap(),
        )
        .text_ex(340, 170, true, Rotation::D270, 'n', "5321D25B")
        .print_many(1);

    stream.write_all(&label.clone().data())?;

    Ok(())
}
