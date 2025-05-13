use crate::high_level::pdf_pagehandler::PdfPageHandle;
use crate::pdf_elements::colors::RgbColors;

#[derive(Debug, Clone)]
pub struct PdfCell {
    pub text: String,
}

#[derive(Debug)]
pub struct PdfTable {
    pub x: i32,
    pub y: i32,
    pub cell_height: i32,
    pub rows: usize,
    pub cols: usize,
    pub total_width: i32,
    pub column_widths: Vec<i32>, // ancho por columna
    pub data: Vec<Vec<PdfCell>>,
    pub border_color: RgbColors,
    pub text_color: RgbColors,
}

impl PdfTable {
    pub fn new(x: i32, y: i32, total_width: i32, cell_height: i32, rows: usize, cols: usize) -> Self {
        Self {
            x,
            y,
            cell_height,
            rows,
            cols,
            total_width,
            column_widths: vec![total_width / (cols as i32); cols],
            data: vec![vec![PdfCell { text: "".to_string() }; cols]; rows],
            border_color: RgbColors::Black,
            text_color: RgbColors::Black,
        }
    }

    pub fn set_column_widths(&mut self, percentages: &[f32]) {
        assert_eq!(percentages.len(), self.cols);
        let sum: f32 = percentages.iter().sum();
        assert!((sum - 100.0).abs() < 0.01, "Los porcentajes deben sumar 100");

        self.column_widths = percentages
            .iter()
            .map(|p| ((self.total_width as f32) * (p / 100.0)).round() as i32)
            .collect();
    }

    pub fn set_cell_text(&mut self, row: usize, col: usize, text: &str) {
        if row < self.rows && col < self.cols {
            self.data[row][col].text = text.to_string();
        }
    }

    pub fn push_to_page(&self, page: &mut PdfPageHandle) {
        let mut content = String::new();
        content += &self.border_color.to_pdf_RG();
        content += "\n";

        for row in 0..self.rows {
            let mut x_cursor = self.x;
            for col in 0..self.cols {
                let y = self.y - (row as i32) * self.cell_height;
                let w = self.column_widths[col];
                content += &format!(
                    "{} {} {} {} re S\n",
                    x_cursor, y - self.cell_height, w, self.cell_height
                );
                x_cursor += w;
            }
        }

        content += &self.text_color.to_pdf_rg();
        content += "\n/F1 10 Tf\n";

        for row in 0..self.rows {
            let mut x_cursor = self.x;
            for col in 0..self.cols {
                let text = &self.data[row][col].text;
                let x = x_cursor + 2;
                let y = self.y - (row as i32) * self.cell_height - 12;
                content += &format!("BT {} {} Td ({}) Tj ET\n", x, y, Self::escape(text));
                x_cursor += self.column_widths[col];
            }
        }

        page.add_raw(&content);
    }

    fn escape(s: &str) -> String {
        s.replace('\\', r"\\").replace('(', r"\(").replace(')', r"\)")
    }
}
