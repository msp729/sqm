use clap::{Parser, ValueEnum};
use crossterm::{
    terminal::{size, Clear, ClearType},
    QueueableCommand,
};
use std::{
    fmt::{Display, Formatter},
    io::{stdout, Result, Write},
    thread::sleep,
    time::Duration,
};

#[derive(Debug, Clone, ValueEnum)]
enum Pattern {
    XOR,
    XNOR,
}

impl Pattern {
    fn pat(&self, x: u16, t: u16) -> u16 {
        match self {
            Self::XOR => x ^ t,
            Self::XNOR => x ^ !t,
        }
    }
}

impl Display for Pattern {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::XOR => write!(fmt, "xor"),
            Self::XNOR => write!(fmt, "xnor"),
        }
    }
}

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
    frame: u64,
    /// draw "towers" instead of floating points
    #[arg(short, long, default_value_t = false)]
    tower: bool,
    /// numeric pattern (xor is preferred)
    #[arg(short, long, default_value_t = Pattern::XOR)]
    pattern: Pattern,
    /// character for occupied spaces
    #[arg(short, long, default_value_t = '*')]
    on: char,
    /// number of frames after which to loop back,
    /// or 0 for "do not loop"
    #[arg(short, long, default_value_t = 0)]
    r#loop: u16,
    /// height-wrap (modulus),
    /// or 0 for "do not wrap"
    #[arg(short, long, default_value_t = 0)]
    wrap: u16,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let (w, h) = size()?;
    let mut b = [0; 4];
    let mut stdout = stdout();

    let t0 = args.initialt;
    let x0 = args.leftx;
    let cumulative = args.tower;
    let pat = args.pattern;
    args.on.encode_utf8(&mut b);
    let lo = args.r#loop;
    let wr = args.wrap;

    let mut t = t0;
    let length = Duration::from_micros(100 * args.frame);
    let x1 = x0 + w;
    let xr = x0..x1;
    let xv: Vec<u16> = Vec::from_iter(xr);
    let mut yv = vec![0; w as usize];

    loop {
        for i in 0..(w as usize) {
            yv[i] = pat.pat(xv[i], t);
        }
        let maxy = {
            let m = yv
                .iter()
                .max()
                .expect("This message should only appear in the case of an empty terminal.")
                .clone();
            if wr != 0 && wr < m {
                wr
            } else {
                m
            }
        };
        let miny = yv
            .iter()
            .min()
            .expect("This message should only appear in the case of an empty terminal.")
            .clone();
        for i in 0..(w as usize) {
            yv[i] = if wr == 0 {
                ((yv[i] - miny) as u32 * h as u32) / maxy as u32
            } else {
                (((yv[i] - miny) % wr) as u32 * h as u32) / maxy as u32
            } as u16;
        }

        for y in 1..=h {
            let y = h - y;
            for x in 0..w as usize {
                if cumulative {
                    stdout.write(if yv[x] >= y { &b } else { &[32] })?;
                } else {
                    stdout.write(if yv[x] == y { &b } else { &[32] })?;
                }
            }
        }

        stdout.flush()?;
        t = t + 1;
        if t - lo == t0 {
            t = t0
        }
        sleep(length);
        stdout.queue(Clear(ClearType::All))?;
    }
}
