use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Target file extensions
    #[arg(short, long, num_args = 0..)]
    pub extensions: Vec<String>,

    /// Show high priority only
    #[arg(short, long)]
    pub important_only: bool,

    /// Target a directory (from current dir)
    #[arg(short, long)]
    pub dir: Option<String>,

    /// Recursion max depth
    #[arg(short, long)]
    pub recursion: Option<usize>,
}
