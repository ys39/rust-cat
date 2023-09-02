use anyhow::Result;
use clap::Parser;
use std::io::Write;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about="Concatenate FILE(s) to standard output.",
    long_about = None
)]
struct Cli {
    /// equivalent to -vET
    #[clap(short('A'), long)]
    show_all: bool,

    /// number nonempty output lines, overrides -n
    #[clap(short('b'), long)]
    number_nonblank: bool,

    /// equivalent to -vE
    #[clap(short('e'))]
    e: bool,

    /// display $ at end of each line
    #[clap(short('E'), long)]
    show_ends: bool,

    /// number all output lines
    #[clap(short('n'), long)]
    number: bool,

    /// suppress repeated empty output lines
    #[clap(short('s'), long)]
    squeeze_blank: bool,

    /// equivalent to -vT
    #[clap(short('t'))]
    t: bool,

    /// display TAB characters as ^I
    #[clap(short('T'), long)]
    show_tabs: bool,

    /// (ignored)
    #[clap(short('u'))]
    u: bool,

    /// use ^ and M- notation, except for LFD and TAB
    #[clap(short('v'), long)]
    show_nonprinting: bool,

    // The file path to read
    file: Vec<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    let mut content = std::string::String::new();

    // read file
    for path in &args.file {
        let file_content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => {
                eprintln!("cat: {}: No such file or directory.", path.display());
                exit(1);
            }
        };
        content.push_str(&file_content);
    }

    for line in content.lines() {
        writeln!(std::io::stdout(), "{}", line)?;
    }

    Ok(())
}
