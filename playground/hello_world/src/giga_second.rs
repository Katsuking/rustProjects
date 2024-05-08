pub mod giga_second {
    use time::PrimitiveDateTime as Datetime;

    // Return a Datetime one billion seconds after start
    pub fn after(start: Datetime) -> Datetime {
        start + time::Duration::seconds(1_000_000_000)
    }
}
