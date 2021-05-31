#![feature(min_specialization)]

use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::wrap_pyfunction;

use libc::{c_char, c_int};
use std::ffi::CString;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

pub fn nom_hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

pub fn hex_color(input: String) -> Result<Color, String> {
    let (_, color) = nom_hex_color(&input).map_err(|e| e.to_string())?;
    Ok(color)
}

#[repr(C)]
pub struct ResultCTransport<T: Copy + Clone> {
    pub is_ok: bool,
    pub err_msg: *mut c_char,
    pub err_len: c_int,
    pub data: *mut T,
}

/// Destructor for the error message strings return from ResultCTransport types
/// across the FFI.
#[no_mangle]
pub unsafe extern "C" fn destroy_err_msg(err_msg: *mut c_char) {
    if err_msg.is_null() {
        return;
    }

    let c_string = CString::from_raw(err_msg);

    drop(c_string);
}

#[no_mangle]
pub extern "C" fn hex_color_c(input: *const c_char) -> ResultCTransport<Color> {
    if input.is_null() {
        let msg = "FFI could not parse null input";
        let err = CString::new(msg).unwrap();
        ResultCTransport {
            is_ok: false,
            err_msg: err.into_raw(),
            err_len: msg.len() as c_int,
            data: std::ptr::null_mut(),
        }
    } else {
        // In a "real" scenario, we wouldn't want to cut corners like this.
        let input = unsafe { std::ffi::CStr::from_ptr(input) }
            .to_str()
            .expect("Failed to convert to str")
            .to_string();
        match hex_color(input.clone()) {
            Ok(color) => ResultCTransport {
                is_ok: true,
                err_msg: std::ptr::null_mut(),
                err_len: 0,
                data: Box::into_raw(Box::new(color)),
            },
            Err(e) => {
                let err = CString::new(e.clone()).unwrap();
                ResultCTransport {
                    is_ok: false,
                    err_msg: err.into_raw(),
                    err_len: e.len() as c_int,
                    data: std::ptr::null_mut(),
                }
            }
        }
    }
}

#[cfg(feature = "python")]
#[pyfunction]
fn hex_color_py(input: String) -> PyResult<(u8, u8, u8)> {
    let Color { red, green, blue } =
        hex_color(input).map_err(|e| PyErr::new::<pyo3::exceptions::PyTypeError, _>(e))?;
    Ok((red, green, blue))
}

#[cfg(feature = "python")]
#[pymodule]
fn hexnom(_py: Python, m: &PyModule) -> PyResult<()> {
    // Parse Hex Color from String
    m.add_function(wrap_pyfunction!(hex_color_py, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_color() {
        assert_eq!(
            nom_hex_color("#2F14DF"),
            Ok((
                "",
                Color {
                    red: 47,
                    green: 20,
                    blue: 223,
                }
            ))
        );
    }
}
