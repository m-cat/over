//! Module containing functions for formatting output of objects.

use crate::{arr::Arr, obj::Obj, tup::Tup, value::Value, INDENT_STEP};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::One;

// Returns a `String` with the given amount of spaces.
fn indent(amount: usize) -> String {
    " ".repeat(amount)
}

fn get_char_map(ch: char) -> Option<&'static str> {
    match ch {
        '\\' => Some("\\\\"),
        '\"' => Some("\\\""),
        '\'' => Some("\\\'"),
        '$' => Some("\\$"),
        '\n' => Some("\\n"),
        '\r' => Some("\\r"),
        '\t' => Some("\\t"),
        _ => None,
    }
}

fn replace_all(s: &str) -> String {
    let mut string = String::with_capacity(s.len());

    for ch in s.chars() {
        if let Some(s) = get_char_map(ch) {
            string.push_str(s);
        } else {
            string.push(ch);
        }
    }
    string
}

/// Trait for formatting a .over representation of an object.
pub trait Format {
    fn format(&self, full: bool, indent_amt: usize) -> String;
}

impl Format for BigRational {
    fn format(&self, _full: bool, _indent_amt: usize) -> String {
        let frac_fmt = format!("{}", *self);

        if *self.denom() == BigInt::one() {
            format!("{}.0", frac_fmt)
        } else {
            frac_fmt
        }
    }
}

impl Format for String {
    fn format(&self, _full: bool, _indent_amt: usize) -> String {
        format!("\"{}\"", replace_all(self))
    }
}

impl Format for Value {
    fn format(&self, _full: bool, indent_amt: usize) -> String {
        match *self {
            Self::Null => String::from("null"),

            Self::Bool(ref inner) => {
                if *inner {
                    String::from("true")
                } else {
                    String::from("false")
                }
            }

            Self::Int(ref inner) => format!("{}", inner),

            Self::Frac(ref inner) => inner.format(true, indent_amt),
            Self::Str(ref inner) => inner.format(true, indent_amt),
            Self::Arr(ref inner) => inner.format(true, indent_amt),
            Self::Tup(ref inner) => inner.format(true, indent_amt),
            Self::Obj(ref inner) => inner.format(true, indent_amt),
        }
    }
}

impl Format for Arr {
    fn format(&self, full: bool, indent_amt: usize) -> String {
        match self.len() {
            0 => {
                if full {
                    String::from("[]")
                } else {
                    String::new()
                }
            }
            1 => {
                let f = self.get(0).unwrap().format(true, indent_amt);
                if full {
                    format!("[{}]", f)
                } else {
                    f
                }
            }
            _ => {
                let mut s = if full {
                    String::from("[\n")
                } else {
                    String::new()
                };

                self.with_each(|value| {
                    s.push_str(&format!(
                        "{}{}\n",
                        indent(indent_amt),
                        value.format(true, indent_amt + INDENT_STEP)
                    ))
                });

                if full {
                    let actual_indent_amt = if indent_amt == 0 {
                        0
                    } else {
                        indent_amt - INDENT_STEP
                    };
                    s.push_str(&format!("{}]", indent(actual_indent_amt)));
                }
                s
            }
        }
    }
}

impl Format for Tup {
    fn format(&self, full: bool, indent_amt: usize) -> String {
        match self.len() {
            0 => {
                if full {
                    String::from("()")
                } else {
                    String::new()
                }
            }
            1 => {
                let f = self.get(0).unwrap().format(true, indent_amt);
                if full {
                    format!("({})", f)
                } else {
                    f
                }
            }
            _ => {
                let mut s = if full {
                    String::from("(\n")
                } else {
                    String::new()
                };

                self.with_each(|value| {
                    s.push_str(&format!(
                        "{}{}\n",
                        indent(indent_amt),
                        value.format(true, indent_amt + INDENT_STEP)
                    ))
                });

                if full {
                    s.push_str(&format!("{})", indent(indent_amt - INDENT_STEP)));
                }
                s
            }
        }
    }
}

impl Format for Obj {
    fn format(&self, full: bool, indent_amt: usize) -> String {
        if self.is_empty() && !self.has_parent() {
            if full {
                String::from("{}")
            } else {
                String::new()
            }
        } else {
            let mut s = if full {
                String::from("{\n")
            } else {
                String::new()
            };

            if let Some(parent) = self.get_parent() {
                s.push_str(&format!(
                    "{}^: {}\n",
                    indent(indent_amt),
                    parent.format(true, indent_amt + INDENT_STEP)
                ));
            }

            self.with_each(|field, value| {
                s.push_str(&format!(
                    "{}{}: {}\n",
                    indent(indent_amt),
                    field,
                    value.format(true, indent_amt + INDENT_STEP)
                ));
            });

            if full {
                s.push_str(&format!("{}}}", indent(indent_amt - INDENT_STEP)));
            }
            s
        }
    }
}
