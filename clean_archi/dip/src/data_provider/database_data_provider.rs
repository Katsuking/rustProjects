use super::DataProvider;

pub struct DatabaseDataProvider {
    connection_string: String,
}

impl DatabaseDataProvider {
    pub fn new(connection_string: &str) -> Self {
        DatabaseDataProvider {
            connection_string: connection_string.to_string(),
        }
    }
}

impl DataProvider for DatabaseDataProvider {
    fn fetch_data(&self) -> String {
        format!("Data from Databasae: {}", self.connection_string)
    }
}
