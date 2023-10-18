//! # Blazor Website
//! CSCI 1260 website project for a C# Blazor Website
//!
//! Uses [`leptos`] under the hood
//!
//! ## Design
//! This website is a single-page-application utilizing several layers of fine-grained reactivity
//! to re-render the [entire body](body::Body) or just a [singular
//! character](utils::TypedText).

pub mod album;
pub mod body;
pub mod header;
pub mod main_menu;
pub mod section;
pub mod song;
pub mod utils;
