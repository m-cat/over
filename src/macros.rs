//! Module containing crate macros.

/// Given an array of elements, converts each element to values and returns an `Arr` containing a
/// vector of the elements. For a non-panicking version, see `try_arr_vec`.
///
/// # Panics
/// Panics if the types don't check out.
#[macro_export]
macro_rules! arr_vec {
    [] => {
        $crate::arr::Arr::new()
    };
    [ $( $elem:expr ),+ , ] => {
        // Rule with trailing comma.
        try_arr_vec![$( $elem ),+].unwrap()
    };
    [ $( $elem:expr ),+ ] => {
        try_arr_vec![$( $elem ),+].unwrap()
    };
}

/// Given an array of elements, converts each element to values and returns an `Arr` containing a
/// vector of the values. Returns an `OverResult` instead of panicking on error. To create an empty
/// `Arr`, use `arr_vec` as it will never fail.
#[macro_export]
macro_rules! try_arr_vec {
    [ $( $elem:expr ),+ , ] => {
        // Rule with trailing comma.
        try_arr_vec![$( $elem ),+]
    };
    [ $( $elem:expr ),+ ] => {
        {
            use $crate::arr::Arr;

            Arr::from_vec(vec![ $( $elem.into() ),+ ])
        }
    };
}

/// Given an array of elements, converts each element to values and returns a `Tup` containing a
/// vector of the values.
#[macro_export]
macro_rules! tup_vec {
    ( $( $elem:expr ),* , ) => {
        tup_vec!($( $elem ),*)
    };
    ( $( $elem:expr ),* ) => {
        {
            use $crate::tup::Tup;

            Tup::from_vec(vec![ $( $elem.into() ),+ ])
        }
    };
}

/// Given an array of field/value pairs, returns an `Obj` containing each pair.
#[macro_export]
macro_rules! obj_map {
    { $( $field:expr => $inner:expr ),* , } => {
        // Rule with trailing comma.
        obj_map!{ $( $field => $inner ),* };
    };
    { $( $field:expr => $inner:expr ),* } => {
        {
            use $crate::obj::Obj;

            let mut obj = Obj::new();

            $(
                obj.set($field, $inner.into());
            )*

            obj
        }
    };
}

#[cfg(test)]
mod tests {
    use OverError;
    use obj::Obj;
    use types::Type::*;
    use value::Value;

    #[test]
    fn arr_vec_basic() {
        assert_eq!(
            arr_vec![Value::Int(1.into()), Value::Int(2.into())],
            try_arr_vec![1, 2].unwrap()
        );

        assert_ne!(
            arr_vec![-1, 2],
            try_arr_vec![Value::Int(1.into()), Value::Int(2.into())].unwrap()
        );
    }

    #[test]
    fn try_arr_vec_mismatch() {
        assert_eq!(
            try_arr_vec![arr_vec![1, 1], arr_vec!['c']],
            Err(OverError::ArrTypeMismatch(
                Arr(Box::new(Char)),
                Arr(Box::new(Int)),
            ))
        );
        assert_eq!(
            try_arr_vec![1, 'c'],
            Err(OverError::ArrTypeMismatch(Char, Int))
        );
    }

    #[test]
    fn obj_map_basic() {
        let mut obj = Obj::new();
        obj.set("a", 1.into());
        obj.set("b", arr_vec![1, 2].into());

        assert_eq!(
            obj,
            obj_map!{
            "a" => 1,
            "b" => arr_vec![1, 2]
        }
        );
    }
}
