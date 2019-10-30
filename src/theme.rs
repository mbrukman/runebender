//! Custom color keys used by the editor.

use druid::piet::Color;
use druid::{Env, Key};

pub const TOOLBAR_BG_COLOR: Key<Color> = Key::new("druid.toolbar.bg-color");
pub const TOOLBAR_BG_SELECTED_COLOR: Key<Color> = Key::new("druid.toolbar.bg-selected-color");
pub const TOOLBAR_BG_HOVER_COLOR: Key<Color> = Key::new("druid.toolbar.bg-hover-color");
pub const TOOLBAR_ICON_COLOR: Key<Color> = Key::new("druid.toolbar.icon-color");
pub const TOOLBAR_ICON_SELECTED_COLOR: Key<Color> = Key::new("druid.toolbar.icon-selected-color");
pub const TOOLBAR_BORDER_COLOR: Key<Color> = Key::new("druid.toolbar.border-color");

pub fn setup_env(env: &mut Env) {
    env.set(TOOLBAR_BG_COLOR, Color::rgb8(0xca, 0xca, 0xca));
    env.set(TOOLBAR_BG_SELECTED_COLOR, Color::rgb8(0x1C, 0x6A, 0xFF));
    env.set(TOOLBAR_BG_HOVER_COLOR, Color::rgb8(0x8e, 0x8e, 0x8e));
    env.set(TOOLBAR_ICON_COLOR, Color::rgb8(0x3e, 0x3a, 0x38));
    env.set(TOOLBAR_ICON_SELECTED_COLOR, Color::rgb8(0xde, 0xda, 0xd8));
    env.set(TOOLBAR_BORDER_COLOR, Color::rgb8(0x5e, 0x5a, 0x5a));
}
