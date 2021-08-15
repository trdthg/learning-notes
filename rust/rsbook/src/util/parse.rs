extern crate nanoid;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

use nanoid::nanoid;

pub fn get_titles(path: &str) -> String {
    let md = File::open(path).unwrap();
    let reader = BufReader::new(md);

    let mut is_incode = -1;
    let mut text: String;
    let mut text_len: u8;
    let mut texts: Vec<String> = Vec::new();
    let mut lens: Vec<u8> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();

        // 判断是否位于代码块内部
        if line.starts_with("```") {
            is_incode *= -1;
            continue;
        }
        if is_incode == 1 {
            continue;
        }

        if line.starts_with("#") {
            // 获得标题
            if line.starts_with("# ") {
                // text = line.replace("# ", "");
                text = line.replace("# ", "");
                text_len = 1;
            } else if line.starts_with("## ") {
                // text = line.replace("## ", "");
                text = line.replace("## ", "&nbsp;&nbsp;&nbsp;");
                text_len = 2;
            } else if line.starts_with("### ") {
                // text = line.replace("### ", "");
                text = line.replace("### ", "&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;");
                text_len = 3;
            } else if line.starts_with("#### ") {
                // text = line.replace("#### ", "");
                text = line.replace(
                    "#### ",
                    "&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;",
                );
                text_len = 4;
            } else if line.starts_with("##### ") {
                // text = line.replace("##### ", "");
                text = line.replace(
                    "##### ",
                    "&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;",
                );
                text_len = 5;
            } else {
                // text = line.replace("###### ", "");
                text = line.replace("###### ", "&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;");
                text_len = 6;
            }
            texts.push(text);
            lens.push(text_len);
        }
    }

    let mut buf = String::new();
    println!("{:?}", lens);
    let mut fa_len: u8 = 0;
    let mut index = 0;
    let mut more_count = 0;

    lens.push(0);
    for len in lens {
        if fa_len == 0 {
            fa_len = len;
            continue;
        };
        print!("{} {}", fa_len, len);
        if len > fa_len {
            // more
            println!(" more          {}", texts[index]);
            more_count += 1;
            buf.push_str(&format!(
                r#"<div class="desc" onclick="a(this)"><span id="{}">{}</span></div><div class="more">"#,
                nanoid!(4), texts[index]
            ));
        } else if fa_len == len {
            // item
            println!(" item          {}", texts[index]);
            buf.push_str(&format!(
                r#"<div class="item" ><span id="{}">{}</span></div>"#,
                nanoid!(4), texts[index]
            ));
        } else {
            // item and back
            println!(" item and back {}", texts[index]);
            buf.push_str(&format!(
                r#"<div class="item"><span id="{}">{}</span></div>"#,
                nanoid!(4), texts[index]
            ));
            if len == 0 {
                break;
            }
            more_count -= fa_len - len;
            for _i in [..(fa_len - len)] {
                buf.push_str("</div>");
            }
        }
        index += 1;
        fa_len = len;
    }

    for _i in [..more_count] {
        buf.push_str("</div>");
    }
    buf
}

pub fn get_links(nav: &HashMap<String, String>) -> String {
    // <span><a href="./">Trdthg</a></span>
    let mut res = String::new();
    for (k, v) in nav {
        res.push_str(&format!("<span class = \"nav_lists\"><a href=\"../{}.html\">{}</a></span>", v, k));
    }
    res
}

pub fn get_links_index(nav: &HashMap<String, String>) -> String {
    // <span><a href="./">Trdthg</a></span>
    let mut res = String::new();
    for (k, v) in nav {
        res.push_str(&format!(r#"<span class = "nav_lists"><a href="{}.html">{}</a></span>"#, v, k));
    }
    res
}



