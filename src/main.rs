use std::str;

use rust_patterns::patterns::system_design::state::get_driving_license;

fn main() -> Result<(), ()> {
    get_driving_license();

    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // Byte strings can have byte escapes...
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...but no unicode escapes
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);

    if let Ok(my_str) = str::from_utf8(escaped) {
        println!("And the same as text: '{}'", my_str);
    }

    Ok(())
}
