use crate::value::Value;

/// In memory representation of the list of [`Value`] in one Row
#[derive(Default)]
pub struct Row {
    pub values: Vec<Value>,
}

/// In memory representation of the Rows of one [`Group`]
#[derive(Default)]
pub struct Group {
    pub rows: Vec<Row>,
}

impl Group {
    /// Returns true of this group has no rows
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Returns the number of rows in this group
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

/// In memory representation of the GitQL Object which has titles and groups
#[derive(Default)]
pub struct GitQLObject {
    pub titles: Vec<String>,
    pub groups: Vec<Group>,
}

impl GitQLObject {
    /// Flat the list of current groups into one main group
    pub fn flat(&mut self) {
        let mut rows: Vec<Row> = vec![];
        for group in &mut self.groups {
            rows.append(&mut group.rows);
        }

        self.groups.clear();
        self.groups.push(Group { rows })
    }

    /// Returns true of there is no groups
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    /// Returns the number of groups in this Object
    pub fn len(&self) -> usize {
        self.groups.len()
    }
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
