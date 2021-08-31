
pub fn bit_test() {
    use bitflags::bitflags;

    bitflags! {
        struct MyFlags: u32 {
            const FLAG_A   = 0b00000001;
            const FLAG_B   = 0b00000010;
            const FLAG_C   = 0b00000100;
            const FLAG_ABC = Self::FLAG_A.bits | Self::FLAG_B.bits | Self::FLAG_C.bits;
        }
    }

    impl MyFlags {
        pub fn clear(&mut self) -> &mut MyFlags {
            self.bits = 0;
            self
        }
    }

    impl std::fmt::Display for MyFlags {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:032b}", self.bits)
        }
    }

    let mut flags = MyFlags::FLAG_ABC;
    println!("{}", flags);
    println!("{}", flags.clear());

}


#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {
        bit_test();
    }
}