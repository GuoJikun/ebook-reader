use chardetng::EncodingDetector;
use encoding_rs::Encoding;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Novel {
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub chapters: Vec<Chapter>,
}

/// 主入口：解析小说 txt
pub fn parse_txt(bytes: &[u8]) -> Novel {
    let text = decode_text(bytes);
    let lines: Vec<&str> = text.lines().collect();

    let title = detect_title(&lines);
    let author = detect_author(&lines);
    let description = None;
    let chapters = parse_chapters(&lines);

    Novel { title, author, description, chapters }
}

/// 自动检测文本编码（UTF-8/GBK等），返回 UTF-8 String
fn decode_text(bytes: &[u8]) -> String {
    let mut detector = EncodingDetector::new();
    detector.feed(bytes, true);
    let encoding = detector.guess(None, true);

    let (decoded, _, _) = encoding.decode(bytes);
    decoded.to_string()
}

/// 自动提取小说标题（只提取书名号《》之间的文字）
fn detect_title(lines: &[&str]) -> Option<String> {
    if lines.is_empty() {
        return None;
    }

    // 使用正则表达式提取书名号之间的内容
    let title_re = Regex::new(r"《([^》]+)》").unwrap();

    // 在前12行中查找书名号
    for &line in lines.iter().take(12) {
        let curr_line = line.trim();

        // 排除包含"章"或"回"的行（很可能是章节标题）
        if curr_line.contains('章') || curr_line.contains('回') {
            continue;
        }

        // 提取书名号之间的内容
        if let Some(cap) = title_re.captures(curr_line) {
            let title = cap[1].trim().to_string();
            if !title.is_empty() && title.len() <= 40 {
                return Some(title);
            }
        }
    }

    None
}

/// 提取作者信息
fn detect_author(lines: &[&str]) -> Option<String> {
    // 目前未实现
    if lines.is_empty() {
        return None;
    }

    // 使用正则表达式提取书名号之间的内容
    let author_re = Regex::new(r"作者：(.+)").unwrap();

    // 在前12行中查找书名号
    for &line in lines.iter().take(12) {
        let curr_line = line.trim();

        // 提取作责
        if let Some(cap) = author_re.captures(curr_line) {
            let author = cap[1].trim().to_string();
            if !author.is_empty() && author.len() <= 40 {
                return Some(author);
            }
        }
    }

    None
}

/// 自动识别章节
fn parse_chapters(lines: &[&str]) -> Vec<Chapter> {
    let chapter_re = Regex::new(
        r#"(?m)^(第[0-9零一二三四五六七八九十百千万]+[章节回卷][^\n]*)$|^(Chapter\s+\d+.*)$"#,
    )
    .unwrap();

    let mut chapters = Vec::new();
    let mut current_title = None;
    let mut current_content = String::new();

    for line in lines {
        let trimmed = line.trim();

        if chapter_re.is_match(trimmed) {
            // 保存旧章节
            if let Some(t) = current_title.take() {
                chapters.push(Chapter {
                    title: t,
                    content: current_content.trim().to_string(),
                });
                current_content.clear();
            }

            current_title = Some(trimmed.to_string());
        } else {
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    // 最后一个章节
    if let Some(t) = current_title {
        chapters.push(Chapter {
            title: t,
            content: current_content.trim().to_string(),
        });
    }

    chapters
}
