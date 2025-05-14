use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use bytes::Bytes;
use crate::high_level::pdf_pagehandler::PdfPageHandle;
use crate::models::pdf_pages::PdfPages;
use crate::pdf_elements::pdf_catalog::PdfCatalog;
use crate::pdf_elements::pdf_dictionary::PdfDictionary;
use crate::pdf_elements::pdf_font::PdfFont;
use crate::pdf_elements::pdf_header::PdfHeader;
use crate::pdf_elements::pdf_page::PdfPage;
use crate::pdf_elements::pdf_trailer::PdfTrailer;
use crate::traits::pdf_represent::PdfRepresentatation;

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct PDFDocument {
    file_name: String,
    obj_counter: u64,
    pdf_header: PdfHeader,
    pdf_catalog: PdfCatalog,
    pages: PdfPages,
    pdf_trailer: PdfTrailer,
    body_objects: Vec<(String, u64)>, // (objeto serializado, offset)
    xobject_ids: HashMap<String, u64>,
}

impl PDFDocument {
    #[allow(dead_code)]
    fn next_id(&mut self) -> u64 {
        let id = self.obj_counter;
        self.obj_counter += 1;
        id
    }

    pub fn register_xobject(&mut self, name: &str, id: u64) {
        self.xobject_ids.insert(name.to_string(), id);
    }

    pub fn begin_page(&mut self) -> PdfPageHandle {
        let stream_id = self.next_id();
        let page_id = self.next_id();

        PdfPageHandle {
            stream_id,
            page_id,
            content: String::new(),
            xobjects: Vec::new(),
        }
    }

    pub fn finalize_page(&mut self, page_handle: PdfPageHandle) {
        let stream_len = page_handle.content.len();
        let stream_obj = format!(
            "{id} 0 obj\n<< /Length {} >>\nstream\n{}\nendstream\nendobj",
            stream_len,
            page_handle.content,
            id = page_handle.stream_id
        );
        self.body_objects.push((stream_obj, 0));

        let mut dict = PdfDictionary::new();
        dict.add_value("Font", "<< /F1 3 0 R >>".to_string());

        if !page_handle.xobjects.is_empty() {
            let mut xobj_dict = String::from("<< ");
            for name in &page_handle.xobjects {
                if let Some(&id) = self.xobject_ids.get(name) {
                    xobj_dict.push_str(&format!("/{name} {id} 0 R "));
                }
            }
            xobj_dict.push_str(">>");
            dict.add_value("XObject", xobj_dict);
        }

        let page_ref = format!("{} 0 R", page_handle.page_id);
        let stream_ref = format!("{} 0 R", page_handle.stream_id);

        let page = PdfPage {
            parent: (2, 0),
            media_box: (0, 0, 595, 842),
            crop_box: (0, 0, 595, 842),
            rotate: 0,
            user_unit: 1.0,
            contents_ref: vec![stream_ref],
            resources: dict,
        };
        let page_obj = page.get_wrapped(page_handle.page_id, 0);

        self.pages.add_child(page_ref);
        self.body_objects.push((page_obj, 0));
    }

    pub fn new(name: &str) -> Self {
        let mut doc = Self {
            file_name: name.to_string(),
            obj_counter: 4,
            pdf_header: PdfHeader::new(1, 5),
            pdf_catalog: PdfCatalog::new("2 0 R".to_string()),
            pages: PdfPages::new(),
            pdf_trailer: PdfTrailer::new("1 0 R"),
            body_objects: Vec::new(),
            xobject_ids: HashMap::new(),
        };

        let catalog_wrapped = doc.pdf_catalog.get_wrapped(1, 0);
        let pages_wrapped = doc.pages.get_wrapped(2, 0);
        let font = PdfFont::helvetica("F1");
        let font_wrapped = font.get_wrapped(3, 0);

        doc.body_objects.push((catalog_wrapped, 0));
        doc.body_objects.push((pages_wrapped, 0));
        doc.body_objects.push((font_wrapped, 0));

        doc
    }

    pub fn close(&mut self) {
        let pages_wrapped = self.pages.get_wrapped(2, 0);
        if let Some(pages_obj) = self.body_objects.get_mut(1) {
            pages_obj.0 = pages_wrapped;
        }
    }

    pub fn as_bytes(&mut self) -> Bytes {
        let mut output = String::new();
        let mut offsets = vec![0u64];

        output += &self.pdf_header.get_as_string().0;
        output += "\n";
        let mut pos = output.len() as u64;

        for (obj_str, offset) in self.body_objects.iter_mut() {
            *offset = pos;
            offsets.push(pos);
            output += obj_str;
            output += "\n";
            pos = output.len() as u64;
        }

        let xref_start = pos;
        output += &format!("xref\n0 {}\n", offsets.len());
        output += "0000000000 65535 f \n";
        for offset in offsets.iter().skip(1) {
            output += &format!("{:010} 00000 n \n", offset);
        }

        self.pdf_trailer.set_offsets(offsets);
        self.pdf_trailer.set_xref_offset(xref_start);
        let (trailer_str, _) = self.pdf_trailer.get_as_string();
        output += &trailer_str;

        Bytes::from(output)
    }

    pub fn save_to_file(&mut self) -> std::io::Result<()> {
        let bytes = self.as_bytes();
        let file = File::create(&self.file_name)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&bytes)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::models::tm::Tm;
    use crate::pdf_elements::colors::RgbColors;
    use crate::pdf_elements::pdf_image::PdfImage;
    use crate::pdf_elements::pdf_table::PdfTable;
    use crate::pdf_elements::pdf_text::PdfText;
    use super::*;

    #[test]
    fn generate_pdf_with_image_table_text() {
        let mut doc = PDFDocument::new("output.pdf");

        let image_bytes = std::fs::read("logo.jpeg").expect("No se pudo leer la imagen");
        let tm = Tm { a: 100, b: 0, c: 0, d: 50, e: 50, f: 790 };
        let image = PdfImage::new_jpeg("Im1", 400, 200, image_bytes, tm);

        let image_id = doc.next_id();
        let image_obj = image.to_object(image_id).0;
        doc.body_objects.push((image_obj, 0));
        doc.register_xobject("Im1", image_id);

        let cell_height = 30;
        let mut table = PdfTable::new(50, 730, 400, cell_height, 3, 3);
        table.set_column_widths(&[50.0, 30.0, 20.0]);
        table.set_cell_text(0, 0, "Nombre");
        table.set_cell_text(0, 1, "Edad");
        table.set_cell_text(0, 2, "Ciudad");
        table.set_cell_text(1, 0, "Nacho");
        table.set_cell_text(1, 1, "26");
        table.set_cell_text(1, 2, "Montevideo");
        table.set_cell_text(2, 0, "Ana");
        table.set_cell_text(2, 1, "32");
        table.set_cell_text(2, 2, "Colonia");

        let table_height = cell_height * table.rows as i32;
        let text_y = 730 - table_height - 40;

        let mut text = PdfText::from_td(50, text_y);
        text.set_font("/F1", 12);
        text.set_color(RgbColors::DarkGray);
        text.set_line_spacing(16);
        text.add_line("Este documento fue generado automáticamente.");
        text.add_line("Todos los datos son confidenciales.");

        let mut page = doc.begin_page();
        page.xobjects.push("Im1".to_string());
        image.push_to_page(&mut page);
        table.push_to_page(&mut page);
        text.push_to_page(&mut page);
        doc.finalize_page(page);

        doc.close();
        doc.save_to_file().unwrap();
    }
}


