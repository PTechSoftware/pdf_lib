use crate::traits::pdf_represent::PdfRepresentatation;
#[derive(Debug,Default)]
pub(crate)  struct PdfHeader {
    major : i32,
    minor : i32,
}



impl PdfHeader{
    pub(crate) fn new(major: i32, minor: i32) -> Self {
        Self{
            major,
            minor
        }
    }
}

impl PdfRepresentatation for PdfHeader {
    fn get_as_string(&self) -> (String,u64) {
        let s = format!(
            "%PDF-{}.{}\n%%EOF",
            self.major, self.minor
        );
        let size = s.len() as u64;
        (s, size)
    }

    fn get_wrapped(&self, id: u64, generation: u64) -> String {
        format!("obj {} {}\n {} \nendobject", id, generation, self.get_as_string().0)
    }
}

