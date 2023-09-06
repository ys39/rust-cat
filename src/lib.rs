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
    return word.trim().to_string();
}

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

pub fn show_tabs_content_process(tmp_file_content: String) -> String {
    let mut show_tabs_content = String::new();
    for line in tmp_file_content.lines() {
        show_tabs_content.push_str(&format!("{}\n", line.replace("\t", "^I")));
    }
    show_tabs_content
}

pub fn show_ends_content_process(tmp_file_content: String) -> String {
    let mut show_ends_content = String::new();
    for line in tmp_file_content.lines() {
        show_ends_content.push_str(&format!("{}$\n", line));
    }
    show_ends_content
}
