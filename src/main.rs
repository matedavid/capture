use capture;
use clap::Parser;
use std::path;

#[derive(clap::Subcommand, Debug)]
enum Action {
    Function {
        #[clap(short, long, value_parser)]
        name: String
    },
    Interval {
        #[clap(short, long, value_parser, default_value_t=0)]
        start: usize,

        #[clap(short, long, value_parser, default_value_t=0)]
        end: usize
    },
}

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // File to creates snippet from
    #[clap(short, long, value_parser)]
    file: String,

    // Function to take
    #[clap(subcommand)]
    action: Action,
}

fn main() {
    let args = Args::parse();

    let path = path::Path::new(&args.file);
    let mut cap = match capture::Capture::new(&path) {
        Ok(cap) => cap,
        Err(e) => {
            println!("Error: {}", e); 
            std::process::exit(1);
        }
    };

    match &args.action {
        Action::Function { name } => {
            cap.from_function(name).unwrap();
        },
        Action::Interval { start, end } => { 
            cap.from_interval(*start, *end).unwrap();
        },
    }
    cap.print();
}
