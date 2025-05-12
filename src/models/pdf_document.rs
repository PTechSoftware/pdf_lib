use crate::models::pdf_pages::PdfPages;
use crate::pdf_elements::pdf_catalog::PdfCatalog;
use crate::pdf_elements::pdf_header::PdfHeader;
use crate::pdf_elements::pdf_trailer::PdfTrailer;

pub struct PDFDocument {
    file_name : String,
    obj_counter : u64,
    pdf_header : PdfHeader,
    pdf_catalog : PdfCatalog,
    pages : PdfPages,


    pdf_trailer : PdfTrailer,
    serialize: Vec<String>
}



impl PDFDocument {
    pub fn new(name: &str) -> Self {
        let mut obj_counter = 1_u64;
        


       todo!()
    }
}
