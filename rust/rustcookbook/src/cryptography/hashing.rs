use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
    }
}

pub fn get_SHA256() -> Result<()> {
    let path = "xxx.log";
    std::fs::write(path, "some sentenses")?;
    use std::fs::File;
    use std::io::{BufReader, Read};
    let mut reader = BufReader::new(File::open(path)?);
    use ring::digest::{Context, Digest, SHA256};
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..]);
    }
    let digest: Digest = context.finish();
    println!("{:?}", digest);
    Ok(())
}

pub fn get_hmac_signature() {
    use ring::hmac;
    use ring::rand::SecureRandom;

    // 随机生成一段密钥
    let mut key_value = [0u8; 48];
    let rng = ring::rand::SystemRandom::new();
    rng.fill(&mut key_value).unwrap();

    // 构建出key
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value[..]);

    // 加密
    let message = "this is a message that will be encoded by hmac";
    let signature = hmac::sign(&key, message.as_bytes());
    println!("{:?}", signature);
    // 校验
    hmac::verify(&key, message.as_bytes(), signature.as_ref()).unwrap();
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {
        // get_SHA256();
        get_hmac_signature();
    }
}
