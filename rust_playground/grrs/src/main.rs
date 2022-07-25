use clap::Parser;
/// Comments???
#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}", line)
        }
    }
    println!("Hello, world!");
    println!("{}", args.pattern)
}
// let pattern = std::env::args().nth(1).except("no pattern given");
// let path = std::env::args().nth(2).except("no path given");