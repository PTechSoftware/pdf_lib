use crate::traits::pdf_represent::PdfRepresentatation;
#[derive(Debug,Default)]
pub(crate)  struct PdfHeader {
    major : i32,
    minor : i32,
}



impl PdfHeader{
    #[allow(dead_code)]
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
            "%PDF-{}.{}\n",
            self.major, self.minor
        );
        let size = s.len() as u64;
        (s, size)
    }


    fn get_wrapped(&self, id: u64, generation: u64) -> String {
        let (body, _) = self.get_as_string();
        format!("{id} {generation} obj\n{body}\nendobj")
    }
}

