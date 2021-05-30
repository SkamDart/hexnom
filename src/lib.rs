#![feature(specialization)]

use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[derive(Debug, PartialEq)]
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

#[pyfunction]
fn hex_color_py(input: String) -> PyResult<(u8, u8, u8)> {
    let Color { red, green, blue } =
        hex_color(input).map_err(|e| PyErr::new::<pyo3::exceptions::PyTypeError, _>(e))?;
    Ok((red, green, blue))
}

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
