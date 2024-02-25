use clap::Parser;
use crossterm::{
    terminal::{size, Clear, ClearType},
    QueueableCommand,
};
use std::{
    io::{stdout, Result, Write},
    thread::sleep,
    time::Duration,
};

#[derive(Parser, Debug)]
#[command(version = "0.0.0", about = "a", long_about = "b")]
struct Args {
    /// leftmost x-value
    #[arg(short, long, default_value_t = 0)]
    leftx: u16,
    /// initial t-value
    #[arg(short, long, default_value_t = 0)]
    initialt: u16,
    /// length of frame, in ten-thousandths of a second
    #[arg(short, long, default_value_t = 4_000)]
    frame: u64
}

fn main() -> Result<()> {
    let args = Args::parse();
    let (w, h) = size()?;
    let mut t = args.initialt;
    let x0 = args.leftx;
    let length = Duration::from_micros(100 * args.frame);
    let x1 = x0 + w;
    let mut stdout = stdout();
    let xr = x0..x1;
    let xv: Vec<u16> = Vec::from_iter(xr);
    let mut yv = vec![0; w as usize];
    loop {
        for i in 0..(w as usize) {
            yv[i] = xv[i] ^ t;
        }
        let maxy = yv
            .iter()
            .max()
            .expect("Terminals should have at least one cell")
            .clone();
        for i in 0..(w as usize) {
            yv[i] = ((yv[i] as u32 * h as u32) / maxy as u32) as u16;
        }
        for y in 1..=h {
            let y = h-y;
            for x in 0..w as usize {
                stdout.write_all(if yv[x] >= y { b"0" } else { &[32] })?;
            }
        }
        stdout.flush()?;
        t = t + 1;
        sleep(length);
        stdout.queue(Clear(ClearType::All))?;
    }
}
