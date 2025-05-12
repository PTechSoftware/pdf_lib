// Estructura general del documento
pub struct PdfDocument;
impl PdfDocument {
    pub fn new(_title: &str, _size: (f64, f64)) -> Self {
        Self
    }

    pub fn add_page(&mut self) -> PdfPage {
        PdfPage
    }

    pub fn save(&self, _path: &str) -> std::io::Result<()> {
        std::fs::write(_path, b"%PDF-1.4
1 0 obj
<< /Type /Catalog /Pages 2 0 R >>
endobj
2 0 obj
<< /Type /Pages /Kids [3 0 R] /Count 1 >>
endobj
3 0 obj
<< /Type /Page /Parent 2 0 R /MediaBox [0 0 595 842] /Contents 4 0 R >>
endobj
4 0 obj
<< /Length 44 >>
stream
BT /F1 24 Tf 100 700 Td (Hello, world!) Tj ET
endstream
endobj
xref
0 5
0000000000 65535 f 
0000000009 00000 n 
0000000055 00000 n 
...
trailer
<< /Root 1 0 R /Size 5 >>
startxref
...offset of xref...
%%EOF
")
    }
}

pub struct PdfPage;
impl PdfPage {
    pub fn draw_text(&self, _text: &str, _x: f64, _y: f64, _size: f64, _font: &str) {}
    pub fn draw_image(&self, _img: &str, _x: f64, _y: f64, _w: f64, _h: f64) {}
    pub fn draw_table(&self, _table: super::table::PdfTable, _x: f64, _y: f64) -> std::io::Result<()> {
        Ok(())
    }
}
