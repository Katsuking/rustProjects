use calamine::{open_workbook, Reader, Xlsx, XlsxError};

fn main() {
    let path = "/home/im-outie/Downloads/rust_test.xlsx";
    let mut wb: Xlsx<_> = open_workbook(path).unwrap();

    if let Ok(r) = wb.worksheet_range("Sheet1") {
        for row in r.rows() {
            println!("{:?}, {:?}", row, row[0]);
        }
    }

    // let range = wb.worksheet_range("Sheet1").unwrap_or_else(|e| {
    //     panic!("Failed to open: {:?}", e);
    // });

    // let mut iter = RangeDeserializerBuilder::new().from_range(&range);
}
