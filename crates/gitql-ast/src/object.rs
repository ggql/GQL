use std::collections::HashMap;

use crate::value::Value;

#[derive(Clone)]
pub struct GQLObject {
    pub attributes: HashMap<String, Value>,
}

pub fn flat_gql_groups(groups: &mut Vec<Vec<GQLObject>>) {
    let mut main_group: Vec<GQLObject> = Vec::new();
    for group in groups.iter_mut() {
        main_group.append(group);
    }

    groups.clear();
    groups.push(main_group);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_gql_groups() {
        let mut group: Vec<GQLObject> = Vec::new();
        let mut groups: Vec<Vec<GQLObject>> = Vec::new();

        flat_gql_groups(&mut groups);
        assert_eq!(groups.len(), 1);

        for item in groups.iter() {
            assert_eq!(item.len(), 0);
        }

        group.push(GQLObject {
            attributes: Default::default(),
        });

        groups.clear();
        groups.push(group.to_owned());
        groups.push(group.to_owned());
        assert_eq!(groups.len(), 2);

        flat_gql_groups(&mut groups);
        assert_eq!(groups.len(), 1);

        for item in groups.iter() {
            assert_eq!(item.len(), 2);
        }
    }
}
