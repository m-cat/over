//! Module defining `Object`.

#![allow(unused_imports)] // will complain about num_traits::Zero otherwise

use fraction::Fraction;
use num_traits::Zero;
use parser;
use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::io;
use value::{InnerValue, Value};

/// Object struct.
#[derive(Debug, Default)]
pub struct Object {
    fields: Option<Arc<RwLock<HashMap<String, Value>>>>,
}

impl Object {
    /// Returns a new empty `Object`.
    pub fn new() -> Object {
        Object { Some(Arc::new(RwLock::new(fields: HashMap::new()))) }
    }

    pub fn null() -> Object {
        Object {
            fields: None
        }
    }

    /// Returns a new `Object` loaded from a file.
    pub fn from_file(path: &str) -> io::Result<Object> {
        let mut obj = Self::new();
        parser::load_file(&mut obj, path)?;

        Ok(obj)
    }

    pub fn is_null(&self) -> bool {
        if let None = self.fields {
            true
        } else {
            false
        }
    }

    /// Returns the number of fields for this Object (children not included).
    pub fn size(&self) -> usize {
        self.fields.len()
    }

    /// Removes a field and its associated value from the Object.
    pub fn remove(&mut self, field: &str) {
        self.fields.remove(field);
    }

    /// Clears all fields from the Object.
    pub fn clear(&mut self) {
        self.fields.clear()
    }

    /// Gets the `Value` associated with `field`.
    pub fn value(&self, field: &str) -> Option<&Value> {
        self.fields.get(field)
    }

    /// Sets the `Value` for `field`.
    pub fn set_value(&mut self, field: &str, value: Value) {
        let field = String::from(field);
        self.fields.insert(field, value);
    }

    pub fn set<T>(&mut self, field: &str, inner: T)
    where
        T: InnerValue,
    {
        let field = String::from(field);
        self.fields.insert(field, inner.into_value());
    }

    // /// Gets the value associated with `field` as an int.
    // ///
    // /// # Panics
    // /// If the associated value doesn't exist or is not a uint.
    // pub fn get_int(&self, field: &str) -> Option<int> {
    //     match self.fields.as_ref() {
    //         Some(ref mut vec) => {
    //             // Return the first field in the vector.
    //             match vec[0] {
    //                 Value::Uint(value) => value,
    //                 _ => panic!("Database::get_uint failed: value is not a uint."),
    //             }
    //         }
    //         None => panic!("Database::get_uint failed: no value found."),
    //     }
    // }

    /// An iterator visiting all field/value pairs in arbitrary order.
    pub fn iter(&self) -> Iter<String, Value> {
        self.fields.iter()
    }
}

impl Clone for Object {}
