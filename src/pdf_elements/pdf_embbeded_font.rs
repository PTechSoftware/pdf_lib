#[allow(dead_code)]
pub struct PdfEmbeddedFont {
    pub type_font: String,
    pub subtype_font: String,
    pub name: String,        // nombre de recurso, ej: F1
    pub base_font: String,   // nombre de la font real, ej: CustomFont
    pub font_data: Vec<u8>,  // datos del TTF
    pub font_file_id: i32,   // ID del stream FontFile2
}



use crate::traits::pdf_represent::PdfRepresentatation;
use std::fs;
use std::io::Result;

impl PdfEmbeddedFont {
    #[allow(dead_code)]
    /// Crea una nueva fuente embebida a partir del path al archivo TTF
    pub fn new(name: &str, base_font: &str, path: &str, font_file_id: i32) -> Result<Self> {
        let font_data = fs::read(path)?;
        Ok(Self {
            type_font: "Font".to_string(),
            subtype_font: "TrueType".to_string(),
            name: name.to_string(),         // /F1, /F2, etc.
            base_font: base_font.to_string(), // /CustomFont
            font_data,
            font_file_id,
        })
    }
    #[allow(dead_code)]
    /// Devuelve el objeto FontFile2 (como string)
    pub fn get_font_file2_stream(&self) -> (String, u64) {
        let stream = format!(
            "<< /Length {}
   /Length1 {}
   /Filter /FlateDecode >>
stream
{}
endstream",
            self.font_data.len(),
            self.font_data.len(),
            std::str::from_utf8(&self.font_data).unwrap_or("")
        );
        let len = stream.len() as u64;
        (stream, len)
    }
}

impl PdfRepresentatation for PdfEmbeddedFont {
    
    fn get_as_string(&self) -> (String, u64) {
        let s = format!(
            "<<
  /Type /Font
  /Subtype /TrueType
  /Name /{}
  /BaseFont /{}
  /Encoding /WinAnsiEncoding
  /FontDescriptor <<
    /Type /FontDescriptor
    /FontName /{}
    /Flags 4
    /FontFile2 {} 
  >>
>>",
            self.name,
            self.base_font,
            self.base_font,
            format!("{} 0 R", self.font_file_id)
        );
        let len = s.len() as u64;
        (s, len)
    }

    fn get_wrapped(&self, id: u64, generation: u64) -> String {
        format!("obj {} {}\n {} \nendobject", id, generation, self.get_as_string().0)
    }
}
