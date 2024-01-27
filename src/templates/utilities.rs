#![allow(dead_code)]
use std::cmp::{min};
use chrono::{DateTime, Utc};

pub fn format_date(timestamp: &DateTime<Utc>) -> String {
    timestamp.format("%d.%m.%Y").to_string()
}

pub fn format_position(position: &f64) -> String {
    let seconds = (position % 60f64).round();
    let minutes = ((position / 60f64) % 60f64).floor();
    let hours = ((position / 60f64) / 60f64).floor();
    format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
}

pub fn display_optional(value: &Option<String>) -> String {
    value.to_owned().unwrap_or(String::from(""))
}

pub fn as_integer(number: &i16) -> i16 {
    number.to_owned()
}

pub fn get_percentage(part: &f64, whole: &f64) -> i64 {
    let fraction = part / whole;
    (fraction * 100.0).floor() as i64
}

pub fn get_percentage_from_int(part: &i64, whole: &i64) -> i64{
    get_percentage(&(*part as f64), &(*whole as f64))
}

pub fn format_overall_rating(rating: &f64) -> String {
    format!("{:.2}", rating)
}

pub fn hex_color_to_rgba(hex_color: &str, opacity: &i64) -> String {
    let mut rgba_color = String::from("rgba(");
    let red = &hex_color[1..3];
    let green = &hex_color[3..5];
    let blue = &hex_color[5..7];
    rgba_color.push_str(get_color_from_hex_number(red).to_string().as_str());
    rgba_color.push_str(", ");
    rgba_color.push_str(get_color_from_hex_number(green).to_string().as_str());
    rgba_color.push_str(", ");
    rgba_color.push_str(get_color_from_hex_number(blue).to_string().as_str());
    rgba_color.push_str(", ");
    rgba_color.push_str(opacity.to_string().as_str());
    rgba_color.push_str(")");

    rgba_color
}

pub fn get_color_from_hex_number(hex_number: &str) -> i64 {
    i64::from_str_radix(hex_number, 16).unwrap_or(0)
}

pub fn get_max_init_page(max_page: &i64) -> i64 {
    min(*max_page, 2)
}