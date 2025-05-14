use crate::high_level::pdf_pagehandler::PdfPageHandle;
use crate::models::pdf_document::PDFDocument;
use crate::models::tm::Tm;
use crate::pdf_elements::pdf_image::PdfImage;
use crate::pdf_elements::pdf_table::PdfTable;
use crate::pdf_elements::pdf_text::PdfText;
use crate::pdf_elements::colors::RgbColors;

pub struct TextStyle {
    pub font: &'static str,
    pub font_size: i32,
    pub color: RgbColors,
    pub line_spacing: Option<i32>,
}

impl TextStyle {
    pub fn default() -> Self {
        Self {
            font: "/F1",
            font_size: 12,
            color: RgbColors::Black,
            line_spacing: None,
        }
    }
}

pub struct HighLevelPdf {
    doc: PDFDocument,
    current_page: Option<PdfPageHandle>,
}

impl HighLevelPdf {
    pub fn new(file_name: &str) -> Self {
        Self {
            doc: PDFDocument::new(file_name),
            current_page: None,
        }
    }

    pub fn begin_page(&mut self) {
        self.current_page = Some(self.doc.begin_page());
    }

    pub fn add_text(&mut self, x: i32, y: i32, lines: &[&str], style: &TextStyle) {
        if let Some(page) = &mut self.current_page {
            let mut text = PdfText::from_td(x, y);
            text.set_font(style.font, style.font_size);
            text.set_color(style.color);
            let spacing = style.line_spacing.unwrap_or((style.font_size as f32 * 1.3) as i32);
            text.set_line_spacing(spacing);
            for line in lines {
                text.add_line(*line);
            }
            text.push_to_page(page);
        }
    }

    pub fn add_table(&mut self, x: i32, y: i32, cell_widths: &[f32], rows: Vec<Vec<&str>>) {
        if let Some(page) = &mut self.current_page {
            let cell_height = 30;
            let mut table = PdfTable::new(x, y, cell_widths.iter().sum::<f32>() as i32, cell_height, rows.len(), cell_widths.len());
            table.set_column_widths(cell_widths);
            for (r_idx, row) in rows.iter().enumerate() {
                for (c_idx, text) in row.iter().enumerate() {
                    table.set_cell_text(r_idx, c_idx, text);
                }
            }
            table.push_to_page(page);
        }
    }

    pub fn add_image_jpeg(&mut self, name: &str, x: i32, y: i32, width: u32, height: u32, data: Vec<u8>) {
        if let Some(page) = &mut self.current_page {
            let tm = Tm { a: width as i32, b: 0, c: 0, d: height as i32, e: x, f: y };
            let image = PdfImage::new_jpeg(name, width, height, data, tm);
            let image_id = self.doc.next_id();
            let image_obj = image.to_object(image_id).0;
            self.doc.body_objects.push((image_obj, 0));
            self.doc.register_xobject(name, image_id);
            image.push_to_page(page);
        }
    }

    pub fn add_image_png(&mut self, name: &str, x: i32, y: i32, width: u32, height: u32, data: Vec<u8>) {
        if let Some(page) = &mut self.current_page {
            let tm = Tm { a: width as i32, b: 0, c: 0, d: height as i32, e: x, f: y };
            let image = PdfImage::new_png_raw(name, width, height, data, tm);
            let image_id = self.doc.next_id();
            let image_obj = image.to_object(image_id).0;
            self.doc.body_objects.push((image_obj, 0));
            self.doc.register_xobject(name, image_id);
            image.push_to_page(page);
        }
    }

    pub fn finalize_page(&mut self) {
        if let Some(page) = self.current_page.take() {
            self.doc.finalize_page(page);
        }
    }

    pub fn save(mut self) {
        self.doc.close();
        self.doc.save_to_file().unwrap();
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Read;

    use crate::high_level::high_level::{HighLevelPdf, TextStyle};

    #[test]
    fn test_high_level_with_image() {
        let file_name = "test_output_with_image.pdf";

        // Crear PDF
        let mut pdf = HighLevelPdf::new(file_name);
        pdf.begin_page();

        // Agregar texto
        let style = TextStyle::default();
        pdf.add_text(50, 750, &["Texto de prueba con imagen"], &style);

        // Agregar tabla
        let columnas = vec![50.0, 30.0, 20.0];
        let filas = vec![
            vec!["Col1", "Col2", "Col3"],
            vec!["1", "2", "3"],
        ];
        pdf.add_table(50, 650, &columnas, filas);

        // Agregar imagen JPEG desde archivo
        let mut file = File::open("logo.jpeg")
            .expect("La imagen test_assets/test.jpg no existe");
        let mut image_data = Vec::new();
        file.read_to_end(&mut image_data).unwrap();
        pdf.add_image_jpeg("ImTest", 50, 450, 100, 100, image_data);

        // Finalizar y guardar
        pdf.finalize_page();
        pdf.save();

        // Verificar que el archivo existe
        assert!(
            fs::metadata(file_name).is_ok(),
            "No se generó el archivo PDF"
        );
    }
}



