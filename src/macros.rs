//! Module containing macros.

/// Given a type and an array of elements, converts each element to values with the type and return
/// an Arr containing a vector of the elements. For a non-panicking version, see `try_arr_vec`.
///
/// # Panics
/// Panics if the types don't check out.
#[macro_export]
macro_rules! arr_vec {
    [] => {
        $crate::arr::Arr::new()
    };
    [ $tok:ident; $( $obj:expr ),+ ] => {
        try_arr_vec![$tok; $( $obj ),+].unwrap()
    };
}

/// Given a type and an array of elements, converts each element to values with the type and return
/// an Arr containing a vector of the values. Returns an `OverResult` instead of panicking on error.
#[macro_export]
macro_rules! try_arr_vec {
    [ Bool; $( $obj:expr ),+ ] => {
        {
            use $crate::arr::Arr;
            use $crate::value::Value;

            Arr::from_vec(vec![ $( Value::new_bool($obj) ),+ ])
        }
    };
    [ Int; $( $obj:expr ),+ ] => {
        {
            use $crate::arr::Arr;
            use $crate::value::Value;

            Arr::from_vec(vec![ $( Value::new_int($obj) ),+ ])
        }
    };
    [ Frac; $( $obj:expr ),+ ] => {
        {
            use $crate::arr::Arr;
            use $crate::value::Value;

            Arr::from_vec(vec![ $( Value::new_frac($obj) ),+ ])
        }
    };
    [ Char; $( $obj:expr ),+ ] => {
        {
            use $crate::arr::Arr;
            use $crate::value::Value;

            Arr::from_vec(vec![ $( Value::new_char($obj) ),+ ])
        }
    };
    [ Str; $( $obj:expr ),+ ] => {
        {
            use $crate::arr::Arr;
            use $crate::value::Value;

            Arr::from_vec(vec![ $( Value::new_str($obj) ),+ ])
        }
    };
    [ Arr; $( $obj:expr ),+ ] => {
        {
            use $crate::arr::Arr;
            use $crate::value::Value;

            Arr::from_vec(vec![ $( Value::new_arr($obj) ),+ ])
        }
    };
    [ Tup; $( $obj:expr ),+ ] => {
        {
            use $crate::arr::Arr;
            use $crate::value::Value;

            Arr::from_vec(vec![ $( Value::new_tup($obj) ),+ ])
        }
    };
    [ Obj; $( $obj:expr ),+ ] => {
        {
            use $crate::arr::Arr;
            use $crate::value::Value;

            Arr::from_vec(vec![ $( Value::new_obj($obj) ),+ ])
        }
    };
}
