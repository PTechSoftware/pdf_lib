use crate::traits::pdf_represent::PdfRepresentatation;

#[derive(Debug,Default)]
pub struct PdfTrailer {
    pub root: String,
    pub size: u64,
    pub xref_offset: u64,
    pub offsets: Vec<u64>,
}

impl PdfTrailer {
    #[allow(dead_code)]
    pub fn new(root: impl Into<String>) -> Self {
        Self {
            root: root.into(),
            size: 0,
            xref_offset: 0,
            offsets: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn set_xref_offset(&mut self, offset: u64) {
        self.xref_offset = offset;
    }

    pub fn set_offsets(&mut self, offsets: Vec<u64>) {
        self.size = offsets.len() as u64;
        self.offsets = offsets;
    }
}
impl PdfRepresentatation for PdfTrailer {
    fn get_as_string(&self) -> (String, u64) {
        // XREF HEADER
        let mut out = String::from("");
        // TRAILER
        out += &format!(
            "trailer
<<
  /Root {}
  /Size {}
>>
startxref
{}
%%EOF",
            self.root,
            self.size,
            self.xref_offset
        );

        let len = out.len() as u64;
        (out, len)
    }
    #[allow(dead_code,unused)]
    fn get_wrapped(&self, id: u64, generation: u64) -> String {
        self.get_as_string().0
    }
}
