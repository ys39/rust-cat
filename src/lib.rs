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
