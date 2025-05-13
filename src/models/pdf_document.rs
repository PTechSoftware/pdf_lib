use crate::models::pdf_pages::PdfPages;
use crate::pdf_elements::pdf_catalog::PdfCatalog;
use crate::pdf_elements::pdf_font::PdfFont;
use crate::pdf_elements::pdf_header::PdfHeader;
use crate::pdf_elements::pdf_page::PdfPage;
use crate::pdf_elements::pdf_trailer::PdfTrailer;
use crate::traits::pdf_represent::PdfRepresentatation;
use bytes::Bytes;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::high_level::pdf_pagehandler::PdfPageHandle;
use crate::pdf_elements::pdf_dictionary::PdfDictionary;

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
}

impl PDFDocument {
    #[allow(dead_code)]
    fn next_id(&mut self) -> u64 {
        let id = self.obj_counter;
        self.obj_counter += 1;
        id
    }

    /// Inicia una nueva página y devuelve su handle para agregar contenido
    pub fn begin_page(&mut self) -> PdfPageHandle {
        let stream_id = self.next_id();
        let page_id = self.next_id();

        PdfPageHandle {
            stream_id,
            page_id,
            content: String::new(),
        }
    }

    /// Cierra una página: inserta el contenido y genera el objeto PdfPage
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
    
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        let mut doc = Self::default();
        doc.file_name = name.to_string();
        doc.obj_counter = 4; // Reservamos hasta 3: Catalog, Pages, Font

        doc.pdf_header = PdfHeader::new(1, 5);

        // === 1. Catalog (1 0 obj)
        doc.pdf_catalog = PdfCatalog::new("2 0 R".to_string());
        let catalog_wrapped = doc.pdf_catalog.get_wrapped(1, 0);

        // === 2. Pages (2 0 obj)
        doc.pages = PdfPages::new();
        let pages_wrapped = doc.pages.get_wrapped(2, 0);

        // === 3. Font (/F1, 3 0 obj)
        let font = PdfFont::helvetica("F1");
        let font_wrapped = font.get_wrapped(3, 0);

        // === Push en orden: Catalog, Pages, Font
        doc.body_objects.push((catalog_wrapped, 0)); // 1 0 obj
        doc.body_objects.push((pages_wrapped, 0)); // 2 0 obj
        doc.body_objects.push((font_wrapped, 0)); // 3 0 obj

        // === Trailer
        doc.pdf_trailer = PdfTrailer::new("1 0 R");

        doc
    }


    #[allow(dead_code)]
    pub fn add_new_page_with_text(&mut self, text: &str) {
        let stream_id = self.next_id();
        let stream_ref = format!("{stream_id} 0 R");
        let stream = format!(
            "{stream_id} 0 obj\n<< /Length {} >>\nstream\n{}\nendstream\nendobj",
            text.len(),
            text
        );
        self.body_objects.push((stream, 0));
        let mut dict = PdfDictionary::new();
        dict.add_value("Font", "<< /F1 3 0 R >>".to_string());

        let page_id = self.next_id();
        let page_ref = format!("{page_id} 0 R");
        let page = PdfPage {
            parent: (2, 0),
            media_box: (0, 0, 595, 842),
            crop_box: (0, 0, 595, 842),
            rotate: 0,
            user_unit: 1.0,
            contents_ref: vec![stream_ref],
            resources: dict,
        };
        let page_wrapped = page.get_wrapped(page_id, 0);

        self.pages.add_child(page_ref);
        self.body_objects.push((page_wrapped, 0));
    }

    #[allow(dead_code)]
    pub fn close(&mut self) {
        // Regenerar objeto Pages actualizado (si se agregaron páginas nuevas)
        let pages_wrapped = self.pages.get_wrapped(2, 0);
        if let Some(pages_obj) = self.body_objects.get_mut(1) {
            pages_obj.0 = pages_wrapped;
        }
    }

    #[allow(dead_code)]
    pub fn as_bytes(&mut self) -> Bytes {
        let mut output = String::new();
        let mut offsets = vec![0u64];

        // HEADER
        output += &self.pdf_header.get_as_string().0;
        output += "\n";
        let mut pos = output.len() as u64;

        // OBJETOS
        for (obj_str, offset) in self.body_objects.iter_mut() {
            *offset = pos;
            offsets.push(pos);
            output += obj_str;
            output += "\n";
            pos = output.len() as u64;
        }

        // XREF
        let xref_start = pos;
        output += &format!("xref\n0 {}\n", offsets.len());
        output += "0000000000 65535 f \n";
        for offset in offsets.iter().skip(1) {
            output += &format!("{:010} 00000 n \n", offset);
        }

        // TRAILER
        self.pdf_trailer.set_offsets(offsets);
        self.pdf_trailer.set_xref_offset(xref_start);
        let (trailer_str, _) = self.pdf_trailer.get_as_string();
        output += &trailer_str;

        Bytes::from(output)
    }

    #[allow(dead_code)]
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
    use crate::pdf_elements::pdf_table::PdfTable;
    use super::*;

    #[test]
    fn generate_pdf() {

        let mut doc = PDFDocument::new("output.pdf");

        // Crear tabla 3x3
        let mut table = PdfTable::new(50, 770,400 , 30, 3, 3);
        table.set_column_widths(&[50.00, 30.00,20.00]);
        table.set_cell_text(0, 0, "Nombre");
        table.set_cell_text(0, 1, "Edad");
        table.set_cell_text(0, 2, "Ciudad");
        table.set_cell_text(1, 0, "Nacho");
        table.set_cell_text(1, 1, "26");
        table.set_cell_text(1, 2, "Montevideo");
        table.set_cell_text(2, 0, "Ana");
        table.set_cell_text(2, 1, "32");
        table.set_cell_text(2, 2, "Colonia");

        // Agregar una página
        let mut page = doc.begin_page();
        table.push_to_page(&mut page);
        doc.finalize_page(page);

        // Cerrar y guardar
        doc.close();
        doc.save_to_file().unwrap();
    }

}
