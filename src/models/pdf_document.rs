use std::fs::File;
use std::io::{BufWriter, Write};
use bytes::Bytes;
use crate::models::pdf_pages::PdfPages;
use crate::pdf_elements::pdf_catalog::PdfCatalog;
use crate::pdf_elements::pdf_font::PdfFont;
use crate::pdf_elements::pdf_header::PdfHeader;
use crate::pdf_elements::pdf_trailer::PdfTrailer;
use crate::traits::pdf_represent::PdfRepresentatation;

#[derive(Debug,Default)]
#[allow(dead_code)]
pub struct PDFDocument {
    file_name : String,
    obj_counter : u64,
    pdf_header : PdfHeader,
    pdf_catalog : PdfCatalog,
    pages : PdfPages,
    pdf_trailer : PdfTrailer,
    body_objects: Vec<(String, u64)>
}

#[allow(dead_code)]

impl PDFDocument {
    #[allow(dead_code)]
    fn next_id(&mut self) -> u64 {
        let id = self.obj_counter;
        self.obj_counter += 1;
        id
    }
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        let mut doc = Self::default();
        doc.file_name = name.to_string();
        doc.obj_counter = 1;

        // HEADER
        doc.pdf_header = PdfHeader::new(1, 4);

        // FONT
        let font = PdfFont::helvetica("F1");
        let font_id = doc.next_id();
        let font_ref = format!("{} 0 R", font_id);
        let font_wrapped = font.get_wrapped(font_id, 0);
        doc.body_objects.push((font_wrapped, 0));

        // SIMPLE CONTENT STREAM (Hello PDF)
        let content = "BT /F1 24 Tf 100 700 Td (Hello PDF) Tj ET".to_string();
        let stream_id = doc.next_id();
        let stream_ref = format!("{} 0 R", stream_id);
        let stream = format!(
            "{} 0 obj\n<< /Length {} >>\nstream\n{}\nendstream\nendobj",
            stream_id,
            content.len(),
            content
        );
        doc.body_objects.push((stream, 0));

        // PAGE
        let page_id = doc.next_id();
        let page_ref = format!("{} 0 R", page_id);
        let resources = format!("<< /Font << /F1 {} >> >>", font_ref);
        let page = format!(
            "{} 0 obj\n<< /Type /Page /Parent {} /MediaBox [0 0 595 842] /Contents {} /Resources {} >>\nendobj",
            page_id, "2 0 R", stream_ref, resources
        );
        doc.body_objects.push((page, 0));

        // PAGES
        let pages_id = doc.next_id(); // debe ser 2
        let pages = format!(
            "{} 0 obj\n<< /Type /Pages /Kids [{}] /Count 1 >>\nendobj",
            pages_id, page_ref
        );
        doc.body_objects.push((pages, 0));

        // CATALOG
        let catalog_id = doc.next_id(); // debe ser 1
        let catalog = format!(
            "{} 0 obj\n<< /Type /Catalog /Pages {} >>\nendobj",
            catalog_id, "2 0 R"
        );
        doc.body_objects.push((catalog, 0));

        // TRAILER (lo completamos en close)
        doc.pdf_trailer = PdfTrailer::new("1 0 R");

        doc
    }

    #[allow(dead_code)]
    pub fn close(&mut self) {
        let mut output = String::new();
        let mut offsets = vec![0u64]; // entrada 0 obligatoria

        // HEADER
        output += &self.pdf_header.get_as_string().0;
        output += "\n";
        let mut pos = output.len() as u64;

        // OBJETOS
        for (_, (obj_str, _)) in self.body_objects.iter_mut().enumerate() {
            offsets.push(pos);
            output += obj_str;
            output += "\n";
            pos = output.len() as u64;
        }

        // XREF
        let xref_start = pos;
        output += &format!("xref\n0 {}\n", offsets.len());
        output += "0000000000 65535 f \n"; // entrada 0
        for offset in offsets.iter().skip(1) {
            output += &format!("{:010} 00000 n \n", offset);
        }

        // TRAILER
        self.pdf_trailer.set_offsets(offsets);
        self.pdf_trailer.set_xref_offset(xref_start);
        let (trailer_str, _) = self.pdf_trailer.get_as_string();
        output += &trailer_str;

    }
    #[allow(dead_code)]
    pub fn as_bytes(&mut self) -> Bytes {
        let mut output = String::new();
        let mut offsets = vec![0u64];
        output += &self.pdf_header.get_as_string().0;
        output += "\n";
        let mut pos = output.len() as u64;

        for (obj_str, _) in self.body_objects.iter_mut() {
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
    use super::*;

    #[test]
    fn generate_hello_pdf() {
        let mut doc = PDFDocument::new("test_hello.pdf");
        doc.close(); // genera to_do internamente
        doc.save_to_file().expect("No se pudo guardar el PDF");
        assert!(std::path::Path::new("test_hello.pdf").exists());
    }
}

