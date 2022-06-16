// File type mask values
// TODO: Incomplete types based on this documentation:  https://man7.org/linux/man-pages/man7/inode.7.html
const S_IFMT: u32 = 0o0170000;
const S_IFREG: u32 = 0o0100000;
const S_IFDIR: u32 = 0o0040000;

pub fn mode_to_string(mode_bits: u32) -> String {
    let mut output = String::new();

    // Calculate file type
    let type_bits: u32 = mode_bits & S_IFMT;
    match type_bits {
        S_IFREG => output.push_str("-"),
        S_IFDIR => output.push_str("d"),
        _ => output.push_str("?"),
    }

    // Calculate permissions
    let permission_bits = mode_bits & 0o7777;
    output.push_str(&format!("{:o}", permission_bits));

    output
}