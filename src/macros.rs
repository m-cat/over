//! Macros.

/// Returns the given field of the given Object as the given type.
///
/// A set of one or more `obj` Objects can be provided. If `field` is not found in the first `obj`
/// then an attempt will be made to retrieve it from the rest of the set. If it's still not found,
/// we return an error.
#[macro_export]
macro_rules! obj_get {
    ( $type:ident, $field:expr, $( $obj:expr )=>+ ) => {
        {
            use $crate::value::Value;

            let null = Value::Null;
            let mut res = Ok(&null);
            let mut temp = Err(Error::UnknownError);

            $(
                if let Err(_) = temp {
                    res = match $obj.value($field) {
                        Some(value) => Ok(value),
                        None => Err(Error::FieldNotFound(String::from($field))),
                    };
                    temp = match res {
                        Ok(&Value::$type(ref inner)) => Ok(inner.clone()),
                        Ok(_) => Err(Error::WrongTypeFound(String::from($field))),
                        Err(e) => Err(e),
                    };
                };
            )+

                temp
        }
    }
}
