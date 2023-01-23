use askama::Result;
use chrono::Duration;
use humansize::format_size;

fn pluralize(input: &str, count: i64) -> String {
    if count == 1 {
        input.to_string()
    } else {
        input.to_string() + "s"
    }
}

// TODO: this would be nice and easy to test
pub fn humanduration(duration: &Duration) -> Result<String> {
    let duration = *duration;
    if duration < Duration::minutes(1) {
        let seconds = duration.num_seconds();
        return Ok(format!("{} {}", seconds, pluralize("second", seconds)));
    }
    if duration < Duration::hours(1) {
        let minutes = duration.num_minutes();
        return Ok(format!("{} {}", minutes, pluralize("minute", minutes)));
    }
    if duration < Duration::days(1) {
        let hours = duration.num_hours();
        return Ok(format!("{} {}", hours, pluralize("hour", hours)));
    }
    // using weeks is dumb, let's use days up until a month
    if duration < Duration::days(30) {
        let days = duration.num_days();
        return Ok(format!("{} {}", days, pluralize("day", days)));
    }
    // I think most people would assume a month is about 30 days
    let months = duration.num_days() / 30;
    Ok(format!("{} {}", months, pluralize("month", months)))
    // probably no point going past months
}

pub fn humansize(size: &u64) -> Result<String> {
    Ok(format_size(*size, humansize::BINARY))
}
