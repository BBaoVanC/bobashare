use askama::Result;
use chrono::TimeDelta;
use humansize::format_size;

fn pluralize(input: &str, count: i64) -> String {
    if count == 1 {
        input.to_string()
    } else {
        input.to_string() + "s"
    }
}

// TODO: this would be nice and easy to test
pub fn humanduration(duration: &TimeDelta) -> Result<String> {
    let duration = *duration;
    if duration < TimeDelta::try_minutes(1).unwrap() {
        let seconds = duration.num_seconds();
        return Ok(format!("{} {}", seconds, pluralize("second", seconds)));
    }
    if duration < TimeDelta::try_hours(1).unwrap() {
        let minutes = duration.num_minutes();
        return Ok(format!("{} {}", minutes, pluralize("minute", minutes)));
    }
    if duration < TimeDelta::try_days(1).unwrap() {
        let hours = duration.num_hours();
        return Ok(format!("{} {}", hours, pluralize("hour", hours)));
    }
    // using weeks is dumb, let's use days up until a month
    if duration < TimeDelta::try_days(30).unwrap() {
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
