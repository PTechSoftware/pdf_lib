use crate::models::pdf_pages::PdfPages;
use crate::traits::pdf_represent::PdfRepresentatation;

pub struct PdfCatalog {
    ref_pages : PdfPages
}

impl PdfCatalog {
    pub(crate) fn new(pages : PdfPages) -> Self {
        Self{
            ref_pages:pages
        }
    }
}
impl PdfRepresentatation for PdfCatalog {
    fn get_as_string(&self) -> (String, u64) {
        let str = self.ref_pages.get_as_string();
        let out = format!(
            "<< /Type /Catalog\n /Pages {} >>"
            str.0
        );
        let size = out.len() as u64;
        (out, size)
    }
    fn get_wrapped(&self, id: u64, generation: u64) -> String {
        format!("obj {} {}\n {} \nendobject", id, generation, self.get_as_string().0)
    }
}