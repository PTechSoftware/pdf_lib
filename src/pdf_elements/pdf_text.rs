use crate::high_level::pdf_pagehandler::PdfPageHandle;
use crate::models::tm::Tm;
use crate::pdf_elements::colors::RgbColors;

#[allow(dead_code)]
pub(crate) struct PdfText {
    font_name: String,
    font_size: i32,
    lines: Vec<String>,
    td_position: Option<(i32, i32)>,
    tm_position: Option<Tm>,
    line_spacing: i32,
    color: RgbColors,
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
            color: RgbColors::Black,
        }
    }

    pub fn from_td(x: i32, y: i32) -> Self {
        Self {
            td_position: Some((x, y)),
            ..Self::new()
        }
    }

    pub fn from_tm(tm: Tm) -> Self {
        Self {
            tm_position: Some(tm),
            ..Self::new()
        }
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

    pub fn set_color(&mut self, color: RgbColors) {
        self.color = color;
    }

    fn escape_pdf_text(s: &str) -> String {
        s.replace('\\', r"\\").replace('(', r"\(").replace(')', r"\)")
    }

    pub fn to_stream_content(&self) -> String {
        let mut s = String::new();
        s.push_str("BT\n");
        s.push_str(&format!("{}\n", self.color.to_pdf_rg()));
        s.push_str(&format!("{} {} Tf\n", self.font_name, self.font_size));

        if let Some(tm) = self.tm_position {
            s.push_str(&format!("{} Tm\n", tm.to_pdf()));
        } else if let Some((x, y)) = self.td_position {
            s.push_str(&format!("{} {} Td\n", x, y));
        }

        if let Some((first, rest)) = self.lines.split_first() {
            s.push_str(&format!("({}) Tj\n", Self::escape_pdf_text(first)));
            for line in rest {
                s.push_str(&format!("0 -{} Td\n", self.line_spacing));
                s.push_str(&format!("({}) Tj\n", Self::escape_pdf_text(line)));
            }
        }

        s.push_str("ET\n");
        s
    }

    pub fn push_to_page(self, page: &mut PdfPageHandle) {
        page.add_raw(&self.to_stream_content());
    }
}
