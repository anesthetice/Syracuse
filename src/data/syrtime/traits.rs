pub trait TimeFormatting {
    const S_STR_LENGTH: usize = 8;
    const MS_STR_LENGTH: usize = 12;
    fn s_str(self) -> String;
    fn ms_str(self) -> String;
}

impl TimeFormatting for f64 {
    fn s_str(self) -> String {
        let total_s = self as u64;

        let hours = total_s / 3_600;
        let minutes = (total_s % 3_600) / 60;
        let seconds = total_s % 60;

        format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
    }

    fn ms_str(self) -> String {
        let total_ms = (self * 1000.0) as u64;

        let hours = total_ms / 3_600_000;
        let minutes = (total_ms % 3_600_000) / 60_000;
        let seconds = (total_ms % 60_000) / 1000;
        let millis = total_ms % 1000;

        format!("{:0>2}:{:0>2}:{:0>2}.{:0>3}", hours, minutes, seconds, millis)
    }
}

pub trait WeekdayFormatting {
    fn to_string(&self) -> String;
}

impl WeekdayFormatting for jiff::civil::Weekday {
    fn to_string(&self) -> String {
        match self {
            jiff::civil::Weekday::Monday => "Monday",
            jiff::civil::Weekday::Tuesday => "Tuesday",
            jiff::civil::Weekday::Wednesday => "Wednesday",
            jiff::civil::Weekday::Thursday => "Thursday",
            jiff::civil::Weekday::Friday => "Friday",
            jiff::civil::Weekday::Saturday => "Saturday",
            jiff::civil::Weekday::Sunday => "Sunday",
        }
        .to_string()
    }
}
