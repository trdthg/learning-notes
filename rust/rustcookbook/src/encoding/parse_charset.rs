pub fn prercent_toggle_normal() {
    use percent_encoding::{percent_decode, utf8_percent_encode, AsciiSet, CONTROLS};
    const FRAGMENT: AsciiSet = CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
    let input = "confident, <>`\"productive systems programming";

    let iter = utf8_percent_encode(input, &FRAGMENT);
    let encoded: String = iter.collect();
    assert_eq!(
        encoded,
        "confident,%20%3C%3E%60%22productive%20systems%20programming"
    );

    let iter = percent_decode(encoded.as_bytes());
    let decoded = iter.decode_utf8().unwrap();
    assert_eq!(input, decoded);
    println!("before: {}\nafter: {}", input, encoded);
}

pub fn str_toggle_x_www_form_urlencoded() {
    use url::form_urlencoded::{byte_serialize, parse};

    let urlencoded: String = byte_serialize("What is ❤?".as_bytes()).collect();

    let decoded: String = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();

    println!("urlencoded:'{}'", urlencoded);
    println!("decoded:'{}'", decoded);
}

pub fn str_to_0x() {
    use data_encoding::HEXUPPER;

    let original = b"The quick brown fox jumps over the lazy dog.";
    let encoded = HEXUPPER.encode(original);
    let decoded = HEXUPPER.decode(encoded.as_bytes()).unwrap();
    println!("ori: {:?}\ndecoded: {}", decoded, encoded);
    let decoded: String = decoded.iter().map(|&x| x as char).collect();
    println!("ori: {:?}\ndecoded: {}", decoded, encoded);
}

pub fn str_to_base64() {
    use base64;

    let hello = "hello rustaceans";
    let encoded = base64::encode(hello.as_bytes());
    let decoded = base64::decode(encoded.as_bytes());
    println!("ori: {:?}\ndecoded: {}", decoded, encoded);
}

#[test]
pub fn test() {
    prercent_toggle_normal();
    str_toggle_x_www_form_urlencoded();
    str_to_0x();
    str_to_base64();
}
