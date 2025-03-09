use crate::{
    args::coordinator_args::CoordinatorStartArgs, errors::node_space::NodeSpaceError,
    modals::coordinator::Coordinator,
};

pub fn handle_start_coordinator(
    _start_args: &CoordinatorStartArgs,
) -> Result<bool, NodeSpaceError> {
    let coordinator = Coordinator::new();

    coordinator.start()
}
