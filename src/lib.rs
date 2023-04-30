use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use std::collections::HashMap;

macro_rules! define_entity {
    ($entity_name:ident, $($fielf_name:ident: $field_type:ty),*) => {
        #[derive(FromRow, Serialize, Deserialize, Debug)]
        pub struct $entity_name {
            $(pub $fielf_name: $field_type),*
        }

        impl Entity for $entity_name {
            type Key = i32;

            fn get_id(&self) -> &Self::Key {
                &self.id
            }
        }

        impl $entity_name {
            pub const TABLE_NAME: &'static srt = stringify!($entity_name);
            let mut columns = Self::get_columns();
            let mut where_clause = String.empty();
            let mut order_by_clause = String.empty();

            pub fn Select(&self) -> $entity_name {
                columns = Self::get_columns();
                &self
            }

            pub fn Select(&self, columns: Vec<&str>) -> $entity_name {
                $entity_name::columns = columns.join(",");
                &self
            }

            pub fn OrderBy(&self, column:&str) -> $entity_name {
                $entity_name::order_by_clause = format!(" ORDER BY {} ASC", column);
                &self
            }

            pub fn OrderByDesc(&self, column:&str) -> $entity_name {
                $entity_name::order_by_clause = format!(" ORDER BY {} DESC", column);
                &self
            }

            pub fn Where<F>(condition: F) -> $entity_name
            where
                F: Fn(&Self) -> String,
            {
                let entity = Self {
                    $( $field_name: Default::default() ),*
                };
                let condition_str = condition(&entity);
                $entity_name::where_clause = format!(" WHERE {}", condition_str);
                &self
            }

            pub fn Where(condition: &str) -> $entity_name {
                $entity_name::where_clause = format!(" WHERE {}", condition);
                &self
            }

            fn get_columns() -> Vec<String> {
                let mut columns = Vec::new();
                let serialized_entity = serde_json::to_string(&Self::default()).unwrap();
                let entity_map: HashMap<String, serde_json::Value> = serde_json::from_str(&serialized_entity).unwrap();
                for (column_name, _) in entity_map.iter() {
                    columns.push(column_name.to_string());
                }
                columns
            }

            fn build_select_query() -> String {
                format!("SELECT {} FROM {} {} {} ", columns.join(", "), $entity_name::TABLE_NAME, $entity_name::where_clause, $entity_name::order_by_clause)
            }
        }
    };
}
