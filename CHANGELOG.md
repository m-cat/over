# Change Log

## [0.3.0] - 2017-11-12

### Added
- `Obj::get_with_source()`.
- `Obj::write_to_file()` implemented.
- New macros: `int!` and `frac!`.
- Commas can now be used in decimals instead of periods.
- `Arr::to_vec()`, `Tup::to_vec()`, and `Obj::to_map().
- Arithmetic on values.
- `Value` now implements `PartialEq` against primitive integers.
- File inclusion for `Obj`, `Str`, `Arr`, and `Tup`.

### Changed
- Error messages improved.
- Public signature of `Obj::get_parent()`.
- The inner type of `Frac` is now `BigRational` instead of `BigFraction`.
- Errors now include the filename.

### Removed
- `Value` no longer implements `From<f32>` or `From<f64>`.
- Dependency on `fraction` crate.

## [0.2.0] - 2017-10-31

### Changed
- The inner type of `Int` is now `BigInt`.
- The inner type of `Frac` is now `BigFraction`.
- Macros are now shorter, e.g. `arr!` instead of `arr_vec!`.
- `Obj` now has more ergonomic getter functions such as `get_bool()`.

## [0.1.0] - 2017-10-29

### Added 
- The first official public version of the project was released.
