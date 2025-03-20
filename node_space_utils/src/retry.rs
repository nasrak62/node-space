use std::{thread, time};

use crate::errors::node_space::NodeSpaceError;

const TEN_SECONDS: time::Duration = time::Duration::from_secs(10);
const RETRY_COUNT: i32 = 3;

pub fn retry<T, F>(
    mut function: F,
    delay: Option<time::Duration>,
    retires: Option<i32>,
) -> Result<T, NodeSpaceError>
where
    F: FnMut() -> Result<T, NodeSpaceError>,
{
    let effective_delay = match delay {
        Some(value) => value,
        None => TEN_SECONDS,
    };

    let effective_retries = match retires {
        Some(value) => value,
        None => RETRY_COUNT,
    };

    let mut result = Err(NodeSpaceError::CantStartCoordinator("temp".to_string()));

    for retry_count in 0..effective_retries {
        dbg!(retry_count);

        result = function();

        if result.is_ok() {
            break;
        }

        thread::sleep(effective_delay);
    }

    result
}
