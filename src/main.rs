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
    out: Option<String>,
}

fn main() {
    let args = Args::parse();

    let out_file = match args.out {
        Some(file) => file,
        None => format!("untitled_{}.jpg", chrono::offset::Local::now()),
    };

    // Opens a window and starts the raytracer
    pollster::block_on(run(args.scene, out_file));
}
