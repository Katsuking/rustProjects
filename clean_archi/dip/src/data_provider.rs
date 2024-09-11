pub mod database_data_provider;
pub mod file_data_provider;

// ビジネスロジックが依存するデータアクセスのためのtrait
pub trait DataProvider {
    fn fetch_data(&self) -> String;
}
