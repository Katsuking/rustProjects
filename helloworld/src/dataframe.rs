pub mod my_polars {

    use polars::prelude::*;

    // create dataframe for test
    pub fn create_dataframe() -> DataFrame {
        let values = vec![2, 4, 6, 8, 10];

        Series::new("values", values).into_frame()
    }
}