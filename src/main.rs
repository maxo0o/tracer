use clap::Parser;
use tracer::scene::Scene;

///  A simple raytracer written in Rust. Runs on the CPU only... for now!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The filename containing the scene json blob. ie 'car.json'
    #[arg(short, long)]
    scene: String,
}

fn main() {
    let args = Args::parse();
    let scene = Scene::new(args.scene);
    scene.render();
}
