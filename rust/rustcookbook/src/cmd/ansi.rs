use ansi_term::Colour;

fn color_print() {
    println!(
        "{}\n{}\n{}",
        Colour::Red.paint("This is red in color"),
        Colour::Blue.paint("blue in color"),
        Colour::Green.paint("green in color")
    );
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {
        color_print();
    }
}
