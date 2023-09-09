use anyhow::Result;
use clap::Parser;
use mycat::{
    get_input, number_content_process, show_ends_content_process, show_nonprinting_content_process,
    show_tabs_content_process, squeeze_blank_content_process,
};
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
        let mut i = 1;
        loop {
            let word_content = get_input();
            let mut tmp_file_content = word_content;

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

            if args.show_nonprinting {
                tmp_file_content = show_nonprinting_content_process(tmp_file_content);
            }

            if args.squeeze_blank {
                tmp_file_content = squeeze_blank_content_process(tmp_file_content);
            }

            if args.show_tabs {
                tmp_file_content = show_tabs_content_process(tmp_file_content);
            }

            if args.number || args.number_nonblank {
                let tmp_n = number_content_process(i, args.number_nonblank, tmp_file_content);
                tmp_file_content = tmp_n[0].clone();
                if args.number_nonblank {
                    if !tmp_file_content.trim().is_empty() {
                        i = i + 1;
                    }
                } else {
                    i = i + 1;
                }
            }

            if args.show_ends {
                tmp_file_content = show_ends_content_process(tmp_file_content);
            }

            content.push_str(&tmp_file_content);

            // print content
            for line in content.lines() {
                // 最後の行のみ表示する
                writeln!(std::io::stdout(), "{}", line)?;
            }
            content.clear();
        }
    }

    // read file
    let mut line_number = 0;
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

        if args.show_nonprinting {
            tmp_file_content = show_nonprinting_content_process(tmp_file_content);
        }

        if args.squeeze_blank {
            tmp_file_content = squeeze_blank_content_process(tmp_file_content);
        }

        if args.show_tabs {
            tmp_file_content = show_tabs_content_process(tmp_file_content);
        }

        if args.number || args.number_nonblank {
            let tmp_n = number_content_process(line_number, args.number_nonblank, tmp_file_content);
            tmp_file_content = tmp_n[0].clone();
            line_number = tmp_n[1].parse::<i32>().unwrap();
        }

        if args.show_ends {
            tmp_file_content = show_ends_content_process(tmp_file_content);
        }

        content.push_str(&tmp_file_content);
    }

    // print content
    for line in content.lines() {
        writeln!(std::io::stdout(), "{}", line)?;
    }

    Ok(())
}
