//! Module containing crate macros.

/// Given an int, returns a `BigInt`.
#[macro_export]
macro_rules! int {
    ( $int:expr ) => (
        {
            use num::bigint::BigInt;

            let b: BigInt = $int.into();
            b
        }
    );
}

/// Given two ints, returns a `BigFraction`.
/// This is a convenience macro and should not be used where performance is important.
#[macro_export]
macro_rules! frac {
    ( $int1:expr, $int2:expr ) => (
        {
            use fraction::BigFraction;
            use num::Signed;
            use num::bigint::BigUint;

            #[allow(unknown_lints)]
            #[allow(eq_op)]
            let neg = ($int1 < 0) ^ ($int2 < 0);
            let (int1, int2): (BigUint, BigUint) = (($int1.abs() as u64).into(),
                                                    ($int2.abs() as u64).into());
            if neg {
                BigFraction::new_neg(int1, int2)
            } else {
                BigFraction::new(int1, int2)
            }
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
        $crate::arr::Arr::new()
    };
    [ $( $elem:expr ),+ , ] => {
        // Rule with trailing comma.
        try_arr![$( $elem ),+].unwrap()
    };
    [ $( $elem:expr ),+ ] => {
        try_arr![$( $elem ),+].unwrap()
    };
}

/// Given an array of elements, converts each element to values and returns an `Arr` containing a
/// vector of the values. Returns an `OverResult` instead of panicking on error. To create an empty
/// `Arr`, use `arr!` as it will never fail.
#[macro_export]
macro_rules! try_arr {
    [ $( $elem:expr ),+ , ] => {
        // Rule with trailing comma.
        try_arr![$( $elem ),+]
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
macro_rules! tup {
    ( $( $elem:expr ),* , ) => {
        tup!($( $elem ),*)
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
macro_rules! obj {
    { $( $field:expr => $inner:expr ),* , } => {
        // Rule with trailing comma.
        obj!{ $( $field => $inner ),* };
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
                Arr(Box::new(Char)),
                Arr(Box::new(Int)),
            ))
        );
        assert_eq!(try_arr![1, 'c'], Err(OverError::ArrTypeMismatch(Char, Int)));
    }

    #[test]
    fn obj_basic() {
        let mut obj = Obj::new();
        obj.set("a", 1.into());
        obj.set("b", arr![1, 2].into());

        assert_eq!(
            obj,
            obj!{
            "a" => 1,
            "b" => arr![1, 2]
        }
        );
    }
}
