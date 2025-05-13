#[derive(Debug, Clone, Copy)]
pub enum RgbColors {
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Orange,
    Gray,
    LightGray,
    DarkGray,
    Pink,
    Purple,
    Brown,
    Gold,
    Silver,
    Navy,
    Teal,
    Lime,
    Maroon,
    Olive,
    Aqua,
    Custom(f32,f32,f32)
}

impl RgbColors {
    pub fn to_rgb(self) -> (f32, f32, f32) {
        match self {
            RgbColors::Black       => (0.0, 0.0, 0.0),
            RgbColors::White       => (1.0, 1.0, 1.0),
            RgbColors::Red         => (1.0, 0.0, 0.0),
            RgbColors::Green       => (0.0, 1.0, 0.0),
            RgbColors::Blue        => (0.0, 0.0, 1.0),
            RgbColors::Yellow      => (1.0, 1.0, 0.0),
            RgbColors::Cyan        => (0.0, 1.0, 1.0),
            RgbColors::Magenta     => (1.0, 0.0, 1.0),
            RgbColors::Orange      => (1.0, 0.65, 0.0),
            RgbColors::Gray        => (0.5, 0.5, 0.5),
            RgbColors::LightGray   => (0.75, 0.75, 0.75),
            RgbColors::DarkGray    => (0.25, 0.25, 0.25),
            RgbColors::Pink        => (1.0, 0.75, 0.8),
            RgbColors::Purple      => (0.5, 0.0, 0.5),
            RgbColors::Brown       => (0.6, 0.4, 0.2),
            RgbColors::Gold        => (1.0, 0.84, 0.0),
            RgbColors::Silver      => (0.75, 0.75, 0.75),
            RgbColors::Navy        => (0.0, 0.0, 0.5),
            RgbColors::Teal        => (0.0, 0.5, 0.5),
            RgbColors::Lime        => (0.0, 1.0, 0.0),
            RgbColors::Maroon      => (0.5, 0.0, 0.0),
            RgbColors::Olive       => (0.5, 0.5, 0.0),
            RgbColors::Aqua        => (0.0, 1.0, 1.0),
            RgbColors::Custom(r,g,b)  => (r,g,b),
        }
    }

    /// Devuelve el comando PDF `r g b rg` como string
    pub fn to_pdf_rg(self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("{:.3} {:.3} {:.3} rg", r, g, b)
    }

    /// Para trazo (`RG`)
    pub fn to_pdf_RG(self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("{:.3} {:.3} {:.3} RG", r, g, b)
    }
}
