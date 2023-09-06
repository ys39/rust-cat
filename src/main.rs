use anyhow::Result;
use clap::Parser;
use mycat::{
    get_input, show_ends_content_process, show_nonprinting_content_process,
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
        loop {
            let word_content = get_input();
            println!("{}", word_content);
        }
    }

    /*
    let mut file_contents: Vec<String> = Vec::new();
    for path in &args.file {
        // 配列にファイルの内容を書き込む
        let mut tmp: String = std::string::String::new();
        tmp = match std::fs::read_to_string(&path) {
            // ファイルの読み込みが成功した場合 (Ok バリアントが返された場合)、
            // content にはファイルの内容が入り、この内容をそのまま返す
            Ok(content) => content.lines().map(|x| x.to_string()).collect(),
            // ファイルの読み込みが失敗した場合 (Err バリアントが返された場合)、
            // エラーを標準エラー出力に出力してプログラムを終了する
            Err(_) => {
                eprintln!("cat: {}: No such file or directory.", path.display());
                exit(1);
            }
        };
        file_contents.push(tmp);
    }

    for file_content in file_contents {
        println!("{}", file_content);
    }
    */
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
        /*
        日本語の場合、多バイト文字であるため、UTF-8エンコーディングのバイトシーケンスに変換される
        $ echo 'あ' | cat -v
        E3 81 82
        M-cM-^AM-^B
        */
        if args.show_nonprinting {
            tmp_file_content = show_nonprinting_content_process(tmp_file_content);
        }

        // -s
        // -s が指定された場合は、連続する空行を 1 行にまとめる
        if args.squeeze_blank {
            tmp_file_content = squeeze_blank_content_process(tmp_file_content);
        }

        // -T
        // -T が指定された場合は、タブ文字を ^I に置換する
        if args.show_tabs {
            tmp_file_content = show_tabs_content_process(tmp_file_content);
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
