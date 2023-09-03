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
    let mut args: Cli = Cli::parse();

    let mut content = std::string::String::new();

    // 空の場合は標準入力から読み込み、出力する
    // ctrl-c で終了する(終了コードは 130)
    if args.file.is_empty() {
        println!("Please input text.");
        exit(130)
    }

    // read file
    for path in &args.file {
        //let mut tmp_file_content: String = std::string::String::new();
        let file_content = match std::fs::read_to_string(&path) {
            // ファイルの読み込みが成功した場合 (Ok バリアントが返された場合)、
            // content にはファイルの内容が入り、この内容をそのまま返す
            Ok(content) => content,
            // ファイルの読み込みが失敗した場合 (Err バリアントが返された場合)、
            // エラーを標準エラー出力に出力してプログラムを終了する
            Err(_) => {
                eprintln!("cat: {}: No such file or directory.", path.display());
                exit(1);
            }
        };

        let mut tmp_file_content = file_content;

        // -A (equivalent to -vET)
        if args.show_all {
            args.show_nonprinting = true;
            args.show_ends = true;
            args.show_tabs = true;
        }

        // -e (equivalent to -vE)
        if args.e {
            args.show_nonprinting = true;
            args.show_ends = true;
        }

        // -t (equivalent to -vT)
        if args.t {
            args.show_nonprinting = true;
            args.show_tabs = true;
        }

        // -u (ignored)

        // -v (use ^ and M- notation, except for LFD and TAB)
        if args.show_nonprinting {
            let mut show_nonprinting_content = String::new();
            for line in tmp_file_content.lines() {
                let mut tmp_line = String::new();
                for c in line.chars() {
                    // tab と改行以外の制御文字を ^ で置換する
                    if c.is_control() && c != '\t' && c != '\n' {
                        tmp_line.push('^');
                        tmp_line.push((c as u8 + 64) as char);
                    } else {
                        tmp_line.push(c);
                    }
                }
                show_nonprinting_content.push_str(&format!("{}\n", tmp_line));
            }
            tmp_file_content = show_nonprinting_content;
        }

        // -s
        // -s が指定された場合は、連続する空行を 1 行にまとめる
        if args.squeeze_blank {
            let mut squeeze_blank_content = String::new();
            let mut is_empty = false;
            for line in tmp_file_content.lines() {
                if line.is_empty() {
                    if !is_empty {
                        squeeze_blank_content.push_str("\n");
                        is_empty = true;
                    }
                } else {
                    squeeze_blank_content.push_str(&format!("{}\n", line));
                    is_empty = false;
                }
            }
            tmp_file_content = squeeze_blank_content;
        }

        // -T
        // -T が指定された場合は、タブ文字を ^I に置換する
        if args.show_tabs {
            let mut show_tabs_content = String::new();
            for line in tmp_file_content.lines() {
                show_tabs_content.push_str(&format!("{}\n", line.replace("\t", "^I")));
            }
            tmp_file_content = show_tabs_content;
        }

        // -n, -b
        // -n と -b が同時に指定された場合は -b が優先される
        if args.number || args.number_nonblank {
            let mut line_number = 1;
            let mut number_content = String::new();
            for line in tmp_file_content.lines() {
                if args.number_nonblank && line.is_empty() {
                    number_content.push_str("\n");
                } else {
                    number_content.push_str(&format!("{:>6}\t{}\n", line_number, line));
                    line_number += 1;
                }
            }
            tmp_file_content = number_content;
        }

        // -E
        // -E が指定された場合は、各行の末尾に $ を付与する
        if args.show_ends {
            let mut show_ends_content = String::new();
            for line in tmp_file_content.lines() {
                show_ends_content.push_str(&format!("{}$\n", line));
            }
            tmp_file_content = show_ends_content;
        }

        content.push_str(&tmp_file_content);
    }

    // print content
    for line in content.lines() {
        writeln!(std::io::stdout(), "{}", line)?;
    }

    Ok(())
}
