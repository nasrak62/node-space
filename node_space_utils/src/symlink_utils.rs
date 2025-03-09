use crate::modals::{link_action::LinkAction, package::Package};

pub fn handle_link_candidate(
    linked_packages: &Vec<Package>,
    current_package: &Package,
) -> LinkAction {
    let mut path_exists = false;

    for package in linked_packages.iter() {
        let is_same_path = package.path == current_package.path;

        path_exists = is_same_path;

        let is_same_name = package.name == current_package.name;
        let saved_alias = &package.alias;
        let input_alias = &current_package.alias;

        if input_alias.is_none() && is_same_path {
            return LinkAction::DoNothing;
        }

        if input_alias.is_none() && !is_same_path {
            return LinkAction::LinkSelf;
        }

        let input_alias_match_name = match input_alias {
            Some(ref value) => *value == package.name,
            None => false,
        };

        let both_alias_match = match input_alias {
            Some(ref value) => match saved_alias {
                Some(ref out_value) => value == out_value,
                None => false,
            },
            None => false,
        };

        let no_alias_at_all = input_alias.is_none() && saved_alias.is_none();

        if !is_same_path && (input_alias_match_name || both_alias_match) {
            dbg!("linking to another package");

            return LinkAction::LinkToAnother;
        }

        if is_same_path && is_same_name && (no_alias_at_all || both_alias_match) {
            dbg!("doing nothing");

            return LinkAction::DoNothing;
        }
    }

    if path_exists {
        dbg!("doing nothing");

        return LinkAction::DoNothing;
    }

    dbg!("adding self as linked package");

    return LinkAction::LinkSelf;
}
