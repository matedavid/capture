use capture;
use clap::Parser;
use std::path;

#[derive(clap::Subcommand, Debug)]
enum CaptureType {
    Function {
        /// Name of the function to create bookmark from
        name: String,
    },
    Interval {
        /// Line interval to create bookmark from, format start_line:end_line
        #[clap(default_value = "0:0")]
        interval: String,
    },
}

#[derive(clap::Parser, Debug)]
struct AddCommand {
    /// Name of the new bookmark
    name: String,

    /// File to create bookmark from
    #[clap(short, long, value_parser)]
    file: String,

    /// How to create bookmark
    #[clap(subcommand)]
    action: CaptureType,
}

#[derive(clap::Parser, Debug)]
struct DeleteCommand {
    /// Name of the bookmark to delete
    name: String,
}

#[derive(clap::Parser, Debug)]
struct ListCommand {
    #[clap(long)]
    oneline: bool,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Add(AddCommand),
    Delete(DeleteCommand),
    List(ListCommand),
    Snippet,
}

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

fn main() {
    let args = Args::parse();
}
