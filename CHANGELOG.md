# Change Log

## [0.6.0] - 2017-12-15

### Added
- `iter()` function for `Obj`, `Arr`, and `Tup`.

### Changed
- `Obj`, `Arr`, and `Tup` use `Arc` instead of `Rc` internally, making them Sync-able.
- The `Obj` token is now optional when including object files.

### Removed
- `Obj::to_map()`, `Arr::to_vec()`, `Tup::to_vec()`.

## [0.5.2] - 2017-12-12

### Added
- Tracking of file includes and prevention of cyclic includes.

## [0.5.1] - 2017-11-23

### Changed
- Allow calling `get_frac()` on `Int` values.

## [0.5.0] - 2017-11-23

### Added
- Allow indexing Arr and Tup with dot notation.

## [0.4.1] - 2017-11-18

### Changed
- Fix `Type::most_specific` returning false positives for `has_any`.

## [0.4.0] - 2017-11-18

### Added
- Dot notation for object field access in .over files.
- Add `Obj::map_ref`, `Arr::vec_ref` and `Tup::vec_ref`
- `try_obj!` macro
- `Obj::to_map`
- `Obj::is_valid_field` and `Obj::is_valid_field_char`
- `Type::has_any` and `Type::most_specific`

### Changed
- Update README.
- Whitespace after fields in .over files is no longer mandatory.
- Fix display formatting of container values.
- Fix incorrect `Arr` and `Tup` type calculations with `Any`.
- Change the ways `Obj`, `Arr`, and `Tup` can be initialized
- Rename `Arr::get_type` to `inner_type` and `Tup::get_type` to `inner_type_vec`

### Removed
- Remove all mutation functions on `Obj`, `Arr`, and `Tup`

## [0.3.0] - 2017-11-12

### Added
- `Obj::get_with_source()`.
- `Obj::write_to_file()` implemented.
- New macros: `int!` and `frac!`.
- Commas can now be used in decimals as well as periods.
- `Arr::to_vec()`, `Tup::to_vec()`, and `Obj::to_map()`.
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
