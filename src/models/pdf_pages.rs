use crate::traits::pdf_represent::PdfRepresentatation;
#[derive(Debug,Default)]
pub struct PdfPages {
    /// A count of the pages in the pdf
    count : i32,
    /// Takes the count
    kids : Vec<String>

}

impl PdfPages {
    #[allow(dead_code)]
    pub fn new() -> PdfPages {
        PdfPages{
            count : 0,
            kids : Vec::with_capacity(10)
        }
    }
    #[allow(dead_code)]
    pub fn add_child(&mut self, reference : String) {
        self.count += 1;
        self.kids.push(reference);
    }
}

impl PdfRepresentatation for PdfPages {
    fn get_as_string(&self) -> (String, u64) {
        let refs = self.kids.join(" ");
        let s = format!(
            "<<
/Type /Pages
/Kids [ {} ]
/Count {}
>>",refs, self.count);
        let size = s.len() as u64;
        (refs, size)
    }
    fn get_wrapped(&self, id: u64, generation: u64) -> String {
        format!("obj {} {}\n {} \nendobject", id, generation, self.get_as_string().0)
    }
}




