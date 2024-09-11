use crate::data_provider::DataProvider;

pub struct BusinessLogic<T: DataProvider> {
    provider: T,
}

impl<T: DataProvider> BusinessLogic<T> {
    pub fn new(provider: T) -> Self {
        BusinessLogic { provider }
    }

    pub fn process_data(&self) -> String {
        let data = self.provider.fetch_data();
        format!("Processed: {}", data)
    }
}
