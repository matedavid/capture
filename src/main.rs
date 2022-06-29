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

    /// If the bookmark should include comments or skip them
    #[clap(long)]
    no_comments: bool,

    /// How to create the bookmark
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
    /// Only display name and id of the bookmark
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
        CaptureType::Function { name } => match cap.from_function(&name, !command.no_comments) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Error creating bookmark from function: {}", e);
                return;
            }
        },
        CaptureType::Interval { interval } => {
            let re = regex::Regex::new(r"[0-9]+:[0-9]+").unwrap();
            if !re.is_match(interval) {
                eprintln!("Interval does not have correct format");
                return;
            }

            let (start, end) = interval.split_once(":").unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            if end < start {
                eprintln!("End line must be greater or equal than start line");
                return;
            }

            cap.from_interval(start, end, !command.no_comments).unwrap();
        }
    }

    match cap.bookmark(&command.name) {
        Ok(()) => println!("Successfully created bookmark: '{}'", command.name),
        Err(e) => eprintln!("Error creating bookmark: {}", e),
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
        None => eprintln!("Bookmark '{}' does not exist", name),
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
