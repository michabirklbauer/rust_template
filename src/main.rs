use clap::Parser;
use template::math;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// first number
    #[arg(short, long)]
    xx: i32,

    /// second number
    #[arg(short, long, default_value_t = 2)]
    yy: i32,
}

fn main() {
    let args = Args::parse();

    let x: i32 = args.xx;
    let y: i32 = args.yy;

    let z = math::add_two_i32(x, y);

    println!("The sum of {} and {} is {}.", x, y, z);
}
