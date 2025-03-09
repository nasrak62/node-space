use std::sync::{atomic::AtomicBool, Arc};

use signal_hook::consts::TERM_SIGNALS;

use crate::errors::node_space::NodeSpaceError;

pub fn handle_termination_signals() -> Result<Arc<AtomicBool>, NodeSpaceError> {
    let term = Arc::new(AtomicBool::new(false));

    for sig in TERM_SIGNALS {
        match signal_hook::flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term)) {
            Ok(_) => {}
            Err(error) => return Err(NodeSpaceError::CantPlaceSigTermHandler(error.to_string())),
        };

        match signal_hook::flag::register(*sig, Arc::clone(&term)) {
            Ok(_) => {}
            Err(error) => return Err(NodeSpaceError::CantPlaceSigTermHandler(error.to_string())),
        };
    }

    Ok(term)
}
