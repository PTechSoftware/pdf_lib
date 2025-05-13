use crate::traits::pdf_represent::PdfRepresentatation;
pub(crate)  struct PdfFont{
    type_font : String,
    subtype_font : String,
    name : String,
    base_font : String,
}


impl PdfFont {
    #[allow(dead_code)]
    pub fn new(name: &str, base_font: &str) -> Self {
        Self {
            type_font: "Font".to_string(),
            subtype_font: "Type1".to_string(),
            name: name.to_string(),         // por ejemplo "F1"
            base_font: base_font.to_string() // por ejemplo "Helvetica"
        }
    }
    #[allow(dead_code)]
    pub fn helvetica(name: &str) -> Self {
        Self::new(name, "Helvetica")
    }
    #[allow(dead_code)]
    pub fn times(name: &str) -> Self {
        Self::new(name, "Times-Roman")
    }
    #[allow(dead_code)]
    pub fn courier(name: &str) -> Self {
        Self::new(name, "Courier")
    }
    #[allow(dead_code)]
    pub fn symbol(name: &str) -> Self {
        Self::new(name, "Symbol")
    }
    #[allow(dead_code)]
    pub fn zapf_dingbats(name: &str) -> Self {
        Self::new(name, "ZapfDingbats")
    }
}


impl PdfRepresentatation for PdfFont {
    fn get_as_string(&self) -> (String, u64) {
        let s = format!(
            "<<
  /Type /{}
  /Subtype /{}
  /Name /{}
  /BaseFont /{}
  /Encoding /WinAnsiEncoding
>>",
            self.type_font,
            self.subtype_font,
            self.name,
            self.base_font
        );
        (s.clone(), s.len() as u64)
    }

    
    fn get_wrapped(&self, id: u64, generation: u64) -> String {
        let (body, _) = self.get_as_string();
        format!("{id} {generation} obj\n{body}\nendobj")
    }
}
