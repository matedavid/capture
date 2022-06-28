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
struct GetCommand {
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
    /// Create new bookmark
    Add(AddCommand),
    /// Delete an existing bookmark
    Delete(DeleteCommand),
    /// Get an existing bookmark
    Get(GetCommand),
    /// List all saved bookmarks
    List(ListCommand),
    //Snippet,
}

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

fn add_command(command: &AddCommand) {
    let path = path::Path::new(&command.file);

    let mut cap = capture::Capture::new(&path).unwrap();
    match &command.action {
        CaptureType::Function { name } => cap.from_function(&name).unwrap(),
        CaptureType::Interval { interval } => {
            let re = regex::Regex::new(r"[0-9]+:[0-9]+").unwrap();
            if !re.is_match(interval) {
                panic!("Interval does not have correct format");
            }

            let (start, end) = interval.split_once(":").unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            if end < start {
                panic!("End line must be greater or equal than start line");
            }

            cap.from_interval(start, end).unwrap();
        }
    }

    match cap.bookmark(&command.name) {
        Ok(()) => println!("Successfully created bookmark: '{}'", command.name),
        Err(e) => eprintln!("Error creating bookmark: {}", e)
    };
}

fn delete_command(name: &String) {
    match capture::bookmark::delete(&name) {
        Ok(()) => println!("Bookmark '{}' deleted successfully", name),
        Err(e) => eprintln!("Error deleting bookmark: {}", e),
    }
}

fn get_command(name: &String) {
    let bk = capture::bookmark::get_bookmark(&name).unwrap();
    match bk {
        Some(bk) => bk.print(true),
        None => println!("Bookmark '{}' does not exist", name),
    }
}

fn list_command(command: &ListCommand) {
    let bookmarks = capture::bookmark::get_all_bookmarks().unwrap();
    for bk in bookmarks {
        bk.print(!command.oneline);
    }
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Add(command) => add_command(&command),
        Action::Delete(command) => delete_command(&command.name),
        Action::Get(command) => get_command(&command.name),
        Action::List(command) => list_command(&command),
    }
}
