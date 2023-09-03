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
    #[arg(short('A'), long)]
    show_all: bool,

    /// number nonempty output lines, overrides -n
    #[arg(short('b'), long)]
    number_nonblank: bool,

    /// equivalent to -vE
    #[arg(short('e'))]
    e: bool,

    /// display $ at end of each line
    #[arg(short('E'), long)]
    show_ends: bool,

    /// number all output lines
    #[arg(short('n'), long)]
    number: bool,

    /// suppress repeated empty output lines
    #[arg(short('s'), long)]
    squeeze_blank: bool,

    /// equivalent to -vT
    #[arg(short('t'))]
    t: bool,

    /// display TAB characters as ^I
    #[arg(short('T'), long)]
    show_tabs: bool,

    /// (ignored)
    #[arg(short('u'))]
    u: bool,

    /// use ^ and M- notation, except for LFD and TAB
    #[arg(short('v'), long)]
    show_nonprinting: bool,

    // The file path to read
    file: Vec<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    let mut content = std::string::String::new();
    let mut line_number = 1;

    // read file
    for path in &args.file {
        let file_content = match std::fs::read_to_string(&path) {
            // ファイルの読み込みが成功した場合 (Ok バリアントが返された場合)、
            // content にはファイルの内容が入り、この内容をそのまま返す
            Ok(content) => content,
            Err(_) => {
                eprintln!("cat: {}: No such file or directory.", path.display());
                exit(1);
            }
        };

        // add number to each line
        if args.number || args.number_nonblank {
            for line in file_content.lines() {
                if args.number_nonblank && line.is_empty() {
                    content.push_str("\n");
                } else {
                    content.push_str(&format!("{:>6}\t{}\n", line_number, line));
                    line_number += 1;
                }
            }
        } else {
            content.push_str(&file_content);
        }
    }

    // print content
    for line in content.lines() {
        writeln!(std::io::stdout(), "{}", line)?;
    }

    Ok(())
}
