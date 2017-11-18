//! Module containing crate macros.

/// Given an int, returns a `BigInt`.
#[macro_export]
macro_rules! int {
    ( $int:expr ) => (
        {
            use ::num::bigint::BigInt;

            let _b: BigInt = $int.into();
            _b
        }
    );
}

/// Given two ints, returns a `BigRational`.
#[macro_export]
macro_rules! frac {
    ( $int1:expr, $int2:expr ) => (
        {
            ::num::rational::BigRational::new($int1.into(), $int2.into())
        }
    );
}

/// Given an array of elements, converts each element to values and returns an `Arr` containing a
/// vector of the elements. For a non-panicking version, see `try_arr!`.
///
/// # Panics
/// Panics if the types don't check out.
#[macro_export]
macro_rules! arr {
    [] => {
        $crate::arr::Arr::from_vec(vec![]).unwrap()
    };
    [ $( $elem:expr ),+ , ] => {
        // Rule with trailing comma.
        try_arr![ $( $elem ),+ ].unwrap()
    };
    [ $( $elem:expr ),+ ] => {
        try_arr![ $( $elem ),+ ].unwrap()
    };
}

/// Given an array of elements, converts each element to values and returns an `Arr` containing a
/// vector of the values. Returns an `OverResult` instead of panicking on error. To create an empty
/// `Arr`, use `arr!` as it will never fail.
#[macro_export]
macro_rules! try_arr {
    [ $( $elem:expr ),+ , ] => {
        // Rule with trailing comma.
        try_arr![ $( $elem ),+ ]
    };
    [ $( $elem:expr ),+ ] => {
        {
            $crate::arr::Arr::from_vec(vec![ $( $elem.into() ),+ ])
        }
    };
}

/// Given an array of elements, converts each element to values and returns a `Tup` containing a
/// vector of the values.
#[macro_export]
macro_rules! tup {
    ( $( $elem:expr ),* , ) => {
        tup!( $( $elem ),* )
    };
    ( $( $elem:expr ),* ) => {
        {
            $crate::tup::Tup::from_vec(vec![ $( $elem.into() ),+ ])
        }
    };
}

/// Given an array of field/value pairs, returns an `Obj` containing each pair.
/// For a non-panicking version, see `try_obj!`.
///
/// # Panics
/// Panics if a field name is invalid.
#[macro_export]
macro_rules! obj {
    {} => {
        $crate::obj::Obj::from_map_unchecked(::std::collections::HashMap::new())
    };
    { $( $field:expr => $inner:expr ),+ , } => {
        // Rule with trailing comma.
        try_obj!{ $( $field => $inner ),+ }.unwrap()
    };
    { $( $field:expr => $inner:expr ),+ } => {
        try_obj!{ $( $field => $inner ),+ }.unwrap()
    };
}

/// Given a list of field to value pairs, returns an `Obj` with the fields and values.
/// Returns an `OverResult` instead of panicking on error. To create an empty `Obj`, use `obj!` as
/// it will never fail.
#[macro_export]
macro_rules! try_obj {
    { $( $field:expr => $inner:expr ),+ , } => {
        // Rule with trailing comma.
        try_obj!{ $( $field => $inner ),* };
    };
    { $( $field:expr => $inner:expr ),+ } => {
        #[allow(unknown_lints)]
        #[allow(useless_let_if_seq)]
        {
            use $crate::obj::Obj;

            let mut _map = ::std::collections::HashMap::new();
            let mut _parent: Option<$crate::value::Value> = None;

            $(
                if $field == "^" {
                    _parent = Some($inner.into());
                } else {
                    _map.insert($field.into(), $inner.into());
                }
            )*

            match _parent {
                Some(parent) => match parent.get_obj() {
                    Ok(parent) => Obj::from_map_with_parent(_map, parent),
                    e @ Err(_) => e,
                }
                None => Obj::from_map(_map),
            }
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
    fn arr_basic() {
        assert_eq!(
            arr![Value::Int(1.into()), Value::Int(2.into())],
            try_arr![1, 2].unwrap()
        );

        assert_ne!(
            arr![-1, 2],
            try_arr![Value::Int(1.into()), Value::Int(2.into())].unwrap()
        );
    }

    #[test]
    fn try_arr_mismatch() {
        assert_eq!(
            try_arr![arr![1, 1], arr!['c']],
            Err(OverError::ArrTypeMismatch(
                Arr(Box::new(Int)),
                Arr(Box::new(Char)),
            ))
        );
        assert_eq!(try_arr![1, 'c'], Err(OverError::ArrTypeMismatch(Int, Char)));
    }

    #[test]
    fn obj_basic() {
        let obj = Obj::from_map_unchecked(
            map!{"a".into() => 1.into(),
                                               "b".into() => arr![1, 2].into()},
        );

        assert_eq!(
            obj,
            obj!{
                "a" => 1,
                "b" => arr![1, 2]
            }
        );
    }
}
