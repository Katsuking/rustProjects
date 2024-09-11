mod business_loginc;
mod data_provider;

use business_loginc::BusinessLogic;
use data_provider::{
    database_data_provider::DatabaseDataProvider, file_data_provider::FileDataProvider,
};

fn main() {
    // file_data_provider を使用
    let file_provider = FileDataProvider::new("example.txt");
    let file_logi = BusinessLogic::new(file_provider);
    println!("{}", file_logi.process_data());

    // database_data_provider を使用
    let database_provider =
        DatabaseDataProvider::new("適当 Server=myServerAddress;Database=myDataBase;User");
    let database_logi = BusinessLogic::new(database_provider);

    println!("{}", database_logi.process_data())
}
