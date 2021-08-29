use std::fs::File;
use std::io::Error;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Archive;

pub fn compress() {
    // create a file wraped by GzEncoder and Builder
    let path = "tar_name.tar.gz";
    let tar_gz = File::create(path).unwrap();
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    // write all the file under ./for_tar to the "sss" dir in the compressed file
    tar.append_dir_all("sss", "./for_tar").unwrap();
}
pub fn decompress() {
    let path = "tar_name.tar.gz";
    let tar_gz = File::open(path).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".").unwrap();
}

pub fn decompress_choosed_prefix() {
    use std::path::PathBuf;
    let file = File::open("tar_name.tar.gz").unwrap();
    let mut archive = Archive::new(GzDecoder::new(file));
    let prefix = "sss/bundle/logs";

    use error_chain::error_chain;
    error_chain! {
        // 把std Error映射为自定义Error
        foreign_links {
          Io(std::io::Error);
          StripPrefixError(::std::path::StripPrefixError);
        }
        // 自定义错误类型
        errors {
            Single {
                description("MyError!")
                display("Single Error")
            }
            Duple(t: String) {
                description("MyError!")
                display("Dutple {} Error", t)
            }
            Multi(len: u32, data: Vec<u32>) {
                description("MyError!")
                display("Multi len {} data {:?} Error", len, data)
            }
        }
    }

    println!("Extracted the following files:");
    archive
        .entries()
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf> {
            let path = entry.path()?.strip_prefix(prefix)?.to_owned();
            entry.unpack(&path)?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!(">_ => {}", x.display()));
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {
        compress();
        decompress();
        decompress_choosed_prefix();
    }
}
