use std::fs;

#[test]
fn large_file() {
    let file = fs::read_to_string("tests/wayland.xml").unwrap();
    let xml = xml::parse(&file).unwrap();
    eprintln!("{xml:?}");
}
