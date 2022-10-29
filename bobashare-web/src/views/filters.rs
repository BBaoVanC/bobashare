use askama::Result;
use chrono::Duration;
use chrono_humanize::{Accuracy, HumanTime, Tense};
use humansize::format_size;

pub fn humanduration(duration: Duration) -> Result<String> {
    Ok(HumanTime::from(duration).to_text_en(Accuracy::Rough, Tense::Future))
}

pub fn humansize(size: &u64) -> Result<String> {
    Ok(format_size(*size, humansize::BINARY))
}

// TODO: this reference is weird
pub fn roughduration(duration: &Duration) -> Result<String> {
    Ok(HumanTime::from(*duration).to_text_en(Accuracy::Rough, Tense::Present))
}
