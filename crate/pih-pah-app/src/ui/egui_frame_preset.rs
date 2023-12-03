use bevy_egui::egui::{self, containers::Frame, epaint, Color32};

lazy_static::lazy_static! {
  pub static ref TRANSPARENT: Frame = Frame {
    fill: Color32::TRANSPARENT,
    ..Default::default()
  };

  pub static ref NO_SHADOW_FRAME: Frame = Frame {
    shadow: epaint::Shadow::NONE,
    ..Frame::window(&egui::Style::default())
  };
}
