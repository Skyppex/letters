use clap::{ArgGroup, Args, Parser};

/// Write a concise description of the command here.
#[derive(Debug, Clone, Parser)]
#[command(version, author, about)]
#[command(group=ArgGroup::new("log").args(["verbose", "quiet"]).multiple(false))]
#[command(group=ArgGroup::new("from").args(["first", "last"]).multiple(false))]
#[command(group=ArgGroup::new("case").arg("case_sensitive").requires("equals"))]
pub struct LettersArgs {
    /// The source file to read from. If not provided, read from stdin.
    #[arg(short, long)]
    pub source: Option<String>,

    /// The destination file to write to. If not provided, write to stdout.
    #[arg(short, long)]
    pub destination: Option<String>,

    /// Enable verbose logging.
    #[arg(short, long)]
    pub verbose: bool,

    /// Suppress all informational output.
    /// Errors will still be printed to stderr.
    #[arg(short, long)]
    pub quiet: bool,

    /// Get the first n letters from the input. (default 1)
    #[arg(short, long)]
    pub first: Option<Option<u64>>,

    /// Get the last n letters from the input. (default 1)
    #[arg(short, long)]
    pub last: Option<Option<u64>>,

    /// Filter the input to only include letters that contain the given substring.
    #[arg(short, long)]
    pub equals: Option<char>,

    /// Case-sensitive matching.
    #[arg(short = 'C', long)]
    pub case_sensitive: bool,

    #[command(flatten)]
    pub output: Output,
}

#[derive(Debug, Clone, Args)]
#[command(group=ArgGroup::new("format").args(["list", "json"]).multiple(false))]
#[command(group=ArgGroup::new("counting").arg("count").conflicts_with_all(["list", "json", "from"]))]
#[command(group=ArgGroup::new("grouping").arg("group").conflicts_with("from"))]
#[command(group=ArgGroup::new("aggregate").args(["count", "group"]).multiple(false))]
pub struct Output {
    /// Print the result as a list separated by newlines.
    #[arg(short = 'L', long)]
    pub list: bool,

    /// Print the result as a json list.
    #[arg(short = 'j', long)]
    pub json: bool,

    /// Remove punctuation from the output.
    #[arg(short = 'p', long)]
    pub no_punctuation: bool,

    /// Trim whitespace from the output.
    #[arg(short = 't', long)]
    pub trim_whitespace: bool,

    /// Convert input to lowercase.
    #[arg(short = 'w', long)]
    pub lowercase: bool,

    /// Count the number of words in the output.
    #[arg(short = 'n', long)]
    pub count: bool,

    #[arg(short, long)]
    pub group: bool,
}