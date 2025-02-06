use crate::modals::{link_action::LinkAction, package::Package};

pub fn handle_link_candidate(
    linked_packages: &Vec<Package>,
    current_package: &Package,
) -> LinkAction {
    for package in linked_packages.iter() {
        let is_same_path = package.path == current_package.path;
        let is_same_name = package.name == current_package.name;
        let saved_alias = &package.alias;

        let does_current_alias_exist_as_name = match current_package.alias {
            Some(ref value) => *value == package.name,
            None => false,
        };

        let does_alias_exist = match current_package.alias {
            Some(ref value) => match saved_alias {
                Some(ref out_value) => value == out_value,
                None => false,
            },
            None => false,
        };

        if is_same_path && (is_same_name || does_alias_exist) {
            dbg!("doing nothing");

            return LinkAction::DoNothing;
        }

        if !is_same_path && (does_current_alias_exist_as_name || does_alias_exist) {
            dbg!("linking to another package");

            return LinkAction::LinkToAnother;
        }
    }

    dbg!("adding self as linked package");

    return LinkAction::LinkSelf;
}
