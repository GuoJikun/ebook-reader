use std::fs;
use txt_novel_parser::parse_txt;

fn main() {
    let bytes = fs::read("人在南宋，我与蒙古争天下.txt").unwrap();
    let novel = parse_txt(&bytes);

    println!("小说名: {:?}", novel.title);

    for (i, ch) in novel.chapters.iter().enumerate() {
        if i.eq(&0) {
            println!("章节内容: {}", ch.content);
        }
        println!("{}. {}", i + 1, ch.title);
    }
}
