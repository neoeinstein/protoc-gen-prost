mod cases;
#[allow(dead_code)]
mod gen;
pub use cases::CASES;
use prost_validate::Validator;

pub type Factory = Box<dyn Fn() -> (Box<dyn Validator>, i32) + Send + Sync>;

pub fn now() -> i64 {
    time::OffsetDateTime::now_utc().unix_timestamp()
}
