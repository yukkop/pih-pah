//! Pure reusable ui component modules go here. Things like checkbox or a scoreboard, that is not "effectful" (not tied directly to actual data).
use bevy_egui::egui;
use epaint::FontId;

pub fn rich_text(text: impl Into<String>, font: &FontId) -> bevy_egui::egui::RichText {
  egui::RichText::new(text).font(font.clone())
}
