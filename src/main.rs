use clap::Parser;
use tracer::window::run;

///  A simple raytracer written in Rust. Runs on the CPU only... for now!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The filename containing the scene json blob. ie 'car.json'
    #[arg(short, long)]
    scene: String,

    /// The output filename of the render. ie 'car.jpg'
    #[arg(short, long)]
    out: String,
}

fn main() {
    let args = Args::parse();

    // Opens a window and starts the raytracer
    pollster::block_on(run(args.scene, args.out));
}
