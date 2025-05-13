use crate::traits::pdf_represent::PdfRepresentatation;

#[derive(Debug,Default)]
pub struct PdfCatalog {
    ref_pages : String
}

impl PdfCatalog {
    #[allow(dead_code)]
    pub(crate) fn new(pages_ref : String) -> Self {
        Self{
            ref_pages:pages_ref
        }
    }
    #[allow(dead_code)]
    pub(crate) fn set_ref(&mut self, reference : String) {
        self.ref_pages = reference; 
    }
}
impl PdfRepresentatation for PdfCatalog {
    fn get_as_string(&self) -> (String, u64) {
        let r = self.ref_pages.clone();
        let out = format!(
            "<< /Type /Catalog\n /Pages {} >>",
            r
        );
        let size = out.len() as u64;
        (out, size)
    }
    fn get_wrapped(&self, id: u64, generation: u64) -> String {
        format!("obj {} {}\n {} \nendobject", id, generation, self.get_as_string().0)
    }
}