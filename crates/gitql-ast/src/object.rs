use std::error::Error;

use crate::value::Value;
use csv::Writer;

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

    /// Export the GitQLObject as JSON String
    pub fn as_json(&self) -> serde_json::Result<String> {
        let mut elements: Vec<serde_json::Value> = vec![];

        if let Some(group) = self.groups.first() {
            let titles = &self.titles;
            for row in &group.rows {
                let mut object = serde_json::Map::new();
                for (i, value) in row.values.iter().enumerate() {
                    object.insert(
                        titles[i].to_string(),
                        serde_json::Value::String(value.to_string()),
                    );
                }
                elements.push(serde_json::Value::Object(object));
            }
        }

        serde_json::to_string(&serde_json::Value::Array(elements))
    }

    /// Export the GitQLObject as CSV String
    pub fn as_csv(&self) -> Result<String, Box<dyn Error>> {
        let mut writer = Writer::from_writer(vec![]);
        writer.write_record(self.titles.clone())?;
        let row_len = self.titles.len();
        if let Some(group) = self.groups.first() {
            for row in &group.rows {
                let mut values_row: Vec<String> = Vec::with_capacity(row_len);
                for value in &row.values {
                    values_row.push(value.to_string());
                }
                writer.write_record(values_row)?;
            }
        }
        Ok(String::from_utf8(writer.into_inner()?)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_is_empty() {
        let group = Group{ rows: vec![] };

        let ret = group.is_empty();
        assert_eq!(ret, true)
    }

    #[test]
    fn test_group_len() {
        let group = Group{ rows: vec![] };

        let ret = group.len();
        assert_eq!(ret, 0)
    }

    #[test]
    fn test_gitqlobject_flat() {
        let groups = vec![Group{ rows: vec![Row{ values: vec![] }] }];
        let mut object = GitQLObject{ titles: vec![], groups };

        object.flat();
        assert_eq!(object.groups.len(), 1);

        for item in object.groups.iter() {
            assert_eq!(item.len(), 1);
        }

        object.groups.clear();
        object.groups.push(Group{ rows: vec![Row{ values: vec![] }] });
        object.groups.push(Group{ rows: vec![Row{ values: vec![] }] });
        assert_eq!(object.groups.len(), 2);

        object.flat();
        assert_eq!(object.groups.len(), 1);

        for item in object.groups.iter() {
            assert_eq!(item.rows.len(), 2);
        }
    }

    #[test]
    fn test_gitqlobject_is_empty() {
        let object = GitQLObject{ titles: vec![], groups: vec![]};

        let ret = object.is_empty();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_gitqlobject_len() {
        let mut object = GitQLObject{ titles: vec![], groups: vec![]};

        let ret = object.len();
        assert_eq!(ret, 0);

        object.groups.push(Group{ rows: vec![] });

        let ret = object.len();
        assert_eq!(ret, 1);
    }
}
