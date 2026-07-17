use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let giga = 1000000000;
    let mut end:DateTime = start + (Duration::seconds(giga));

    return end;
}
