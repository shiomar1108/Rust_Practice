pub fn is_leap_year(year: u64) -> bool {
    (year.is_multiple_of(4) && year % 100 != 0) || (year.is_multiple_of(400))
}