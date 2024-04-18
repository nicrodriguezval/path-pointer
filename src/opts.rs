use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "path_pointer",
    version = "0.0.1",
    author = "Nicolás Rodríguez",
    about = "CLI app designed to list stored operating system paths for easy navigation"
)]
pub struct Opts {
    pub args: Vec<String>,

    /// List all paths
    #[arg(short, long)]
    list: Option<String>,

    /// Add a new path
    #[arg(short, long)]
    add: Option<String>,

    /// Remove a path
    #[arg(short, long)]
    remove: Option<String>,
}
