use std::fs;
use std::path::Path;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::{path::BaseDirectory, App, Manager};
use tauri_plugin_store::{Store, StoreExt};
use txt_novel_parser::parse_txt;

#[derive(Debug, Serialize, Deserialize)]
struct NovelListItem {
    title: Option<String>,
    md5: Option<String>,
    cover: Option<String>,
    author: Option<String>,
    source_link: Option<String>,
}

pub struct Novel;

impl Novel {
    /// 扫描指定目录下的所有 txt 文件并解析小说信息
    pub fn scan_novels_in_dir(app: &mut App) -> Vec<NovelListItem> {
        let dir_path = Self::get_ebooks_dir(app);
        if dir_path.is_empty() {
            return vec![];
        }

        let path = Path::new(&dir_path);
        if !path.exists() || !path.is_dir() {
            return vec![];
        }

        let mut novels = Vec::new();

        // 读取目录下的所有文件
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let file_path = entry.path();

                // 只处理 .txt 文件
                if file_path.extension().and_then(|s| s.to_str()) == Some("txt") {
                    if let Some(novel_item) = Self::parse_novel_file(&file_path) {
                        novels.push(novel_item);
                    }
                }
            }
        }

        novels
    }

    /// 解析单个小说文件
    fn parse_novel_file(file_path: &Path) -> Option<NovelListItem> {
        // 读取文件内容
        let bytes = fs::read(file_path).ok()?;

        // 计算文件的 MD5
        let md5_hash = format!("{:x}", md5::compute(&bytes));

        // 解析小说信息
        let novel = parse_txt(&bytes);

        Some(NovelListItem {
            title: novel.title,
            md5: Some(md5_hash),
            cover: None, // TODO: 从其他来源获取封面
            author: novel.author,
            source_link: Some(file_path.to_string_lossy().to_string()),
        })
    }

    fn get_local_novel_list() -> Vec<NovelListItem> {
        vec![]
    }

    fn update_local_novel_list(_list: Vec<NovelListItem>) {
        // TODO: 实现更新逻辑
    }

    fn get_ebooks_dir(app: &mut App) -> String {
        let local_store: LocalStore<_> = match LocalStore::new(app) {
            Ok(store) => store,
            Err(_) => return String::new(),
        };
        let path = local_store.get(ConfigKey::EbookDir);
        if path.is_some() {
            return path.unwrap();
        }

        let path = app.path().resolve("ebooks/", BaseDirectory::Resource);

        match path {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => String::new(),
        }
    }

    pub fn new(app: &mut App) {
        // 扫描目录获取所有小说
        let scanned_novels = Self::scan_novels_in_dir(app);
        println!("Scanned novels: {:?}", scanned_novels);
        // 获取之前保存的小说列表
        let mut local_list = Self::get_local_novel_list();

        // 合并新扫描的小说和已有的小说列表
        for scanned in scanned_novels {
            if let Some(md5) = &scanned.md5 {
                // 检查是否已存在（通过 MD5 比对）
                if let Some(pos) = local_list.iter().position(|x| x.md5.as_ref() == Some(md5)) {
                    // 更新已存在的小说信息
                    local_list[pos].title = scanned.title;
                    local_list[pos].author = scanned.author;
                    local_list[pos].source_link = scanned.source_link;
                } else {
                    // 添加新发现的小说
                    local_list.push(scanned);
                }
            } else {
                // 没有 MD5 的直接添加
                local_list.push(scanned);
            }
        }

        Self::update_local_novel_list(local_list);
    }
}

// Config 键的枚举类型，提供类型安全
#[derive(Debug)]
pub enum ConfigKey {
    EbookDir,
}

impl ConfigKey {
    // 将枚举转换为字符串键
    fn as_str(&self) -> &str {
        match self {
            ConfigKey::EbookDir => "ebook_dir",
        }
    }
}

pub struct LocalStore<R: tauri::Runtime> {
    instance: Arc<Store<R>>,
}

impl<R: tauri::Runtime> LocalStore<R> {
    pub fn new(app: &App<R>) -> Result<Self, tauri_plugin_store::Error> {
        let store = app.store("config.bin")?;
        Ok(Self { instance: store })
    }

    // 使用枚举键的类型安全 get 方法
    pub fn get(&self, key: ConfigKey) -> Option<String> {
        self.instance
            .get(key.as_str())
            .and_then(|v| v.as_str().map(|s| s.to_string()))
    }

    // 使用枚举键的类型安全 set 方法
    pub fn set(&self, key: ConfigKey, value: String) {
        self.instance.set(key.as_str(), value);
    }

    // 保存 store 到磁盘
    pub fn save(&self) -> Result<(), tauri_plugin_store::Error> {
        self.instance.save()
    }
}
