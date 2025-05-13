use crate::models::tm::Tm;

/// Módulo de bajo nivel para texto en PDF.
/// Usa `BT ... ET` internamente.
/// Usa `Td` para renglones relativos o `Tm` para transformación absoluta.
#[allow(dead_code)]
pub(crate) struct PdfText {
    font_name: String,       // /F1, /F2, etc.
    font_size: i32,          // tamaño de fuente en puntos
    lines: Vec<String>,      // líneas de texto
    td_position: Option<(i32, i32)>, // si se usa posicionamiento relativo
    tm_position: Option<Tm>,        // si se usa matriz absoluta
    line_spacing: i32,       // distancia entre líneas
}

impl PdfText {
    pub fn new() -> Self {
        Self {
            font_name: "/F1".to_string(),
            font_size: 12,
            lines: Vec::new(),
            td_position: None,
            tm_position: None,
            line_spacing: 14,
        }
    }

    pub fn from_td(x: i32, y: i32) -> Self {
        let mut t = Self::new();
        t.td_position = Some((x, y));
        t
    }

    pub fn from_tm(tm: Tm) -> Self {
        let mut t = Self::new();
        t.tm_position = Some(tm);
        t
    }

    pub fn add_line<S: Into<String>>(&mut self, line: S) {
        self.lines.push(line.into());
    }

    pub fn set_font(&mut self, name: &str, size: i32) {
        self.font_name = name.to_string();
        self.font_size = size;
    }

    pub fn set_line_spacing(&mut self, spacing: i32) {
        self.line_spacing = spacing;
    }

    pub fn to_stream_content(&self) -> String {
        let mut s = String::new();
        s.push_str("BT\n");
        s.push_str(&format!("{} {} Tf\n", self.font_name, self.font_size));

        if let Some(tm) = self.tm_position {
            s.push_str(&format!("{}\n", tm.to_pdf()));
        } else if let Some((x, y)) = self.td_position {
            s.push_str(&format!("{} {} Td\n", x, y));
        }

        if let Some((first, rest)) = self.lines.split_first() {
            s.push_str(&format!("({}) Tj\n", first));
            for line in rest {
                s.push_str(&format!("0 -{} Td\n", self.line_spacing));
                s.push_str(&format!("({}) Tj\n", line));
            }
        }

        s.push_str("ET");
        s
    }
}
