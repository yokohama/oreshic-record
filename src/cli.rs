use clap::{Args, Parser, Subcommand, ArgGroup};

#[derive(Parser, Debug)]
#[command(
    name = "rust-ors",
    version = "1.0",
    author = "yokohama",
    about = "CLI knowledge tool inspired by the Akashic Records.",
    long_about = "OreshicRecord is a CLI-based knowledge management tool \
for engineers working in the terminal.\n\n\
- Record reproducible command logs\n\
- Organize tasks with tracks\n\
- Manage searchable writeups",
    help_template = "

{name} {version} {author}

{about-with-newline}

{usage-heading} {usage}

{all-args}{after-help}
"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    #[command(about = "Record execution command in records/commands/xxx.md")]
    Record(RecordArgs),
    Set(SetArgs),
    Unset,
    Search(SearchArgs),
}

#[derive(Debug, Subcommand)]
pub enum SearchCommands {
    #[command(
        group(
            ArgGroup::new("action")
                .args(["run", "del", "open"])
                .multiple(false)
                .required(false)
        )
    )]
    Command {
        #[arg(
            num_args = 0..=2,
            value_name = "INDEX [SECTION]",
            help = "Up to 2 numbers: <INDEX> [SECTION]"
        )]
        number: Vec<usize>,

        #[arg(long)]
        run: bool,

        #[arg(long)]
        del: bool,

        #[arg(long)]
        open: bool,
    },

    #[command(
        group(
            ArgGroup::new("action")
                .args(["run", "del", "open"])
                .multiple(false)
                .required(false)
        )
    )]
    Track {
        #[arg(
            num_args = 0..=2,
            value_name = "INDEX [SECTION]",
            help = "Up to 2 numbers: <INDEX> [SECTION]"
        )]
        number: Vec<usize>,

        #[arg(long)]
        run: bool,

        #[arg(long)]
        del: bool,

        #[arg(long)]
        open: bool,
    },

    #[command(
        group(
            ArgGroup::new("action")
                .args(["run", "del", "open"])
                .multiple(false)
                .required(false)
        )
    )]
    Query {
        #[arg(required=true)]
        word: String,

        #[arg()]
        number: Option<usize>,

        #[arg(long)]
        run: bool,

        #[arg(long)]
        del: bool,

        #[arg(long)]
        open: bool,
    },

    #[command(
        group(
            ArgGroup::new("action")
                .args(["open"])
                .multiple(false)
                .required(false)
        )
    )]
    Writeup {
        #[arg()]
        number: Vec<usize>,

        #[arg(long)]
        open: bool,
    },
}

#[derive(Debug, Args)]
pub struct RecordArgs {
    /// Do not record command output
    #[arg(short, long)]
    pub quiet: bool,

    /// Use PTY for interactive commands(e.g. ssh, bash, mysql)
    /// or commands that require Ctrl+C to terminate
    #[arg(short, long)]
    pub pty: bool,

    /// Title of the record (used as a query serach target)
    #[arg(short, long)]
    pub title: Option<String>,

    /// Description of the record (used as a query search target)
    #[arg(short, long)]
    pub message: Option<String>,

    /// Tag (e.g. tag1,tag2,tag3; used as a query search target)
    #[arg(long)]
    pub tag: Option<String>,

    /// Command to execute (e.g. ls -la)
    #[arg(required = true, trailing_var_arg=true)]
    pub command: Vec<String>,
    
    // /// History number (e.g. 4580)
    //#[arg(required = true)]
    //pub history_number: String,
}

#[derive(Debug, Args)]
pub struct SetArgs {
    #[arg(required=true)]
    pub track_name: String,
}

#[derive(Debug, Args)]
pub struct SearchArgs {
    #[command(subcommand)]
    pub command: SearchCommands,
}
