use super::DataProvider;

pub struct FileDataProvider {
    file_path: String,
}

impl FileDataProvider {
    pub fn new(file_path: &str) -> Self {
        FileDataProvider {
            file_path: file_path.to_string(),
        }
    }
}

impl DataProvider for FileDataProvider {
    fn fetch_data(&self) -> String {
        // 実際にはファイルデータを読み込む処理を書いたりする
        format!("Data from file: {}", self.file_path)
    }
}
