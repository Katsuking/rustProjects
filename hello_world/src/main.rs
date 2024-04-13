use time::{Date, PrimitiveDateTime as Datetime, Time};
mod clock;
mod giga_second;

fn main() {
    println!("Hello, world!");

    // 日付のインスタンス作成
    let date = Date::from_calendar_date(2024, time::Month::March, 3).expect("Invalid date");
    let time = Time::from_hms(0, 0, 0).expect("Invalid time");

    let start = Datetime::new(date, time);

    // datetimeを渡す
    let result = giga_second::giga_second::after(start);
    println!("One billion seconds later: {}", result);

    // Clock
    let cl = clock::Clock::new(0, 300); // 5 hours
    let new_cl = cl.add_minutes(3);
    println!("時間: {}", new_cl)
}
