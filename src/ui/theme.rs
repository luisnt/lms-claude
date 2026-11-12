use ratatui::style::Color;

pub struct Theme;

impl Theme {
    // Cores baseadas no macOS/iOS Dark Mode
    pub const BACKGROUND: Color = Color::Rgb(28, 28, 30);
    pub const TEXT_PRIMARY: Color = Color::Rgb(245, 245, 247);
    pub const TEXT_SECONDARY: Color = Color::Rgb(142, 142, 147);
    pub const TEXT_BLUE: Color = Color::Rgb(10, 132, 255);
    pub const ACCENT: Color = Color::Rgb(48, 209, 88);
}
