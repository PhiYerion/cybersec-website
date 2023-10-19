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
mod header;
pub use header::Header;

mod main_menu;
pub use main_menu::MainMenuPage;
pub use main_menu::ReturnToMainMenu;

pub mod section;
pub mod song;
pub mod utils;
