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
    [ $( $obj:expr ),+ ] => {
        try_arr_vec![$( $obj ),+].unwrap()
    };
}

/// Given an array of elements, converts each element to values and returns an `Arr` containing a
/// vector of the values. Returns an `OverResult` instead of panicking on error. To create an empty
/// `Arr`, use `arr_vec` as it will never fail.
#[macro_export]
macro_rules! try_arr_vec {
    [ $( $obj:expr ),+ ] => {
        {
            use $crate::arr::Arr;

            Arr::from_vec(vec![ $( $obj.into() ),+ ])
        }
    };
}

/// Given an array of elements, converts each element to values and returns a `Tup` containing a
/// vector of the values. Note that there is no construct to create empty `Tup`s as this is
/// intentionally not supported.
#[macro_export]
macro_rules! tup_vec {
    [ $( $obj:expr ),+ ] => {
        {
            use $crate::tup::Tup;

            Tup::from_vec(vec![ $( $obj.into() ),+ ])
        }
    };
}

#[cfg(test)]
mod tests {
    use OverError;
    use value::Value;

    #[test]
    fn arr_vec_basic() {
        assert_eq!(
            arr_vec![Value::Int(1), Value::Int(2)],
            try_arr_vec![1, 2].unwrap()
        );

        assert_ne!(
            arr_vec![-1, 2],
            try_arr_vec![Value::Int(1), Value::Int(2)].unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn arr_vec_mismatch() {
        let _ = arr_vec![1, 'c'];
    }

    #[test]
    fn try_arr_vec_mismatch() {
        assert_eq!(
            try_arr_vec![arr_vec![1, 1], arr_vec!['c']],
            Err(OverError::ArrTypeMismatch)
        );
    }
}
