use std::fs;
use txt_novel_parser::parse_txt;

fn main() {
    let bytes = fs::read("人在南宋，我与蒙古争天下.txt").unwrap();
    let novel = parse_txt(&bytes);

    println!("作者: {:?}", novel.author);
}
