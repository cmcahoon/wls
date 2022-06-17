// File type mask values
// TODO: Incomplete types based on this documentation:  https://man7.org/linux/man-pages/man7/inode.7.html
const S_IFMT: u32 = 0o0170000;
const S_IFREG: u32 = 0o0100000;
const S_IFDIR: u32 = 0o0040000;

// File permissions mask values
// TODO: Incomplete types based on this documentation: https://man7.org/linux/man-pages/man7/inode.7.html
const S_IRWXU: u32 = 0o00700;
const S_IRUSR: u32 = 0o00400;
const S_IWUSR: u32 = 0o00200;
const S_IXUSR: u32 = 0o00100;
const S_IRWXG: u32 = 0o00070;
const S_IRGRP: u32 = 0o00040;
const S_IWGRP: u32 = 0o00020;
const S_IXGRP: u32 = 0o00010;
const S_IRWXO: u32 = 0o00007;
const S_IROTH: u32 = 0o00004;
const S_IWOTH: u32 = 0o00002;
const S_IXOTH: u32 = 0o00001;

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

    if permission_bits & S_IRWXU == S_IRWXU {
        output.push_str("rwx");
    } else {
        if permission_bits & S_IRUSR == S_IRUSR {
            output.push_str("r");
        } else {
            output.push_str("-");
        }

        if permission_bits & S_IWUSR == S_IWUSR {
            output.push_str("w");
        } else {
            output.push_str("-");
        }

        if permission_bits & S_IXUSR == S_IXUSR {
            output.push_str("x");
        } else {
            output.push_str("-");
        }
    }

    if permission_bits & S_IRWXG == S_IRWXU {
        output.push_str("rwx");
    } else {
        if permission_bits & S_IRGRP == S_IRGRP {
            output.push_str("r");
        } else {
            output.push_str("-");
        }

        if permission_bits & S_IWGRP == S_IWGRP {
            output.push_str("w");
        } else {
            output.push_str("-");
        }

        if permission_bits & S_IXGRP == S_IXGRP {
            output.push_str("x");
        } else {
            output.push_str("-");
        }
    }

    if permission_bits & S_IRWXO != S_IRWXO {
        if permission_bits & S_IROTH == S_IROTH {
            output.push_str("r");
        } else {
            output.push_str("-");
        }

        if permission_bits & S_IWOTH == S_IWOTH {
            output.push_str("w");
        } else {
            output.push_str("-");
        }

        if permission_bits & S_IXOTH == S_IXOTH {
            output.push_str("x");
        } else {
            output.push_str("-");
        }
    } else {
        output.push_str("rwx");
    }

    output
}