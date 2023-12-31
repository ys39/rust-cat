use std::io;

pub fn process_byte(ch: u8) -> Vec<u8> {
    let mut output = Vec::new();

    match ch {
        32..=126 => output.push(ch),
        127 => {
            output.push(b'^');
            output.push(b'?');
        }
        128..=255 => {
            output.push(b'M');
            output.push(b'-');
            if ch >= 128 + 32 {
                if ch < 128 + 127 {
                    output.push(ch - 128);
                } else {
                    output.push(b'^');
                    output.push(b'?');
                }
            } else {
                output.push(b'^');
                output.push(ch - 128 + 64);
            }
        }
        _ => {
            if ch == b'\t' {
                output.push(b'\t');
            } else if ch != b'\n' {
                output.push(b'^');
                output.push(ch + 64);
            }
        }
    }

    output
}

pub fn get_input() -> String {
    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    return word.to_string();
}

// -v (use ^ and M- notation, except for LFD and TAB)
/*
日本語の場合、多バイト文字であるため、UTF-8エンコーディングのバイトシーケンスに変換される
$ echo 'あ' | cat -v
E3 81 82
M-cM-^AM-^B
*/
pub fn show_nonprinting_content_process(tmp_file_content: String) -> String {
    let mut show_nonprinting_content = String::new();
    for line in tmp_file_content.lines() {
        let mut output_bytes = Vec::new();
        for c in line.chars() {
            // 一文字ずつバイトシーケンスに変換して、output_bytes に追加する
            for byte in c.to_string().as_bytes() {
                output_bytes.extend(process_byte(*byte));
            }
        }
        show_nonprinting_content.push_str(&format!(
            "{}\n",
            output_bytes.iter().map(|&x| x as char).collect::<String>()
        ));
    }
    show_nonprinting_content
}

// -s
// -s が指定された場合は、連続する空行を 1 行にまとめる
pub fn squeeze_blank_content_process(tmp_file_content: String) -> String {
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
    squeeze_blank_content
}

// -T
// -T が指定された場合は、タブ文字を ^I に置換する
pub fn show_tabs_content_process(tmp_file_content: String) -> String {
    let mut show_tabs_content = String::new();
    for line in tmp_file_content.lines() {
        show_tabs_content.push_str(&format!("{}\n", line.replace("\t", "^I")));
    }
    show_tabs_content
}

// -n, -b
// -n と -b が同時に指定された場合は -b が優先される
pub fn number_content_process(
    mut line_number: i32,
    number_nonblank: bool,
    tmp_file_content: String,
) -> Vec<String> {
    let mut number_content = String::new();
    if line_number == 0 {
        line_number = 1;
    }
    for line in tmp_file_content.lines() {
        if number_nonblank && line.is_empty() {
            number_content.push_str("\n");
        } else {
            number_content.push_str(&format!("{:>6}\t{}\n", line_number, line));
            line_number += 1;
        }
    }
    let line_number_string = line_number.to_string();
    [number_content, line_number_string].to_vec()
}

// -E
// -E が指定された場合は、各行の末尾に $ を付与する
pub fn show_ends_content_process(tmp_file_content: String) -> String {
    let mut show_ends_content = String::new();
    for line in tmp_file_content.lines() {
        show_ends_content.push_str(&format!("{}$\n", line));
    }
    show_ends_content
}
