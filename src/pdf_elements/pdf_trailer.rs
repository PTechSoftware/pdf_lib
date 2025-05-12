use crate::traits::pdf_represent::PdfRepresentatation;

pub struct PdfTrailer {
    pub root: String,
    pub size: u64,
    pub xref_offset: u64,
    pub offsets: Vec<u64>,
}

impl PdfTrailer {
    pub fn new(root: impl Into<String>) -> Self {
        Self {
            root: root.into(),
            size: 0,
            xref_offset: 0,
            offsets: vec![],
        }
    }

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
        let mut out = String::from("xref\n0 ");
        out += &format!("{}\n", self.size);

        // Entrada 0 es especial: objeto "free"
        out += "0000000000 65535 f \n";

        // El resto son normales
        for offset in &self.offsets[1..] {
            out += &format!("{:010} 00000 n \n", offset);
        }

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
    fn get_wrapped(&self, id: u64, generation: u64) -> String {
        self.get_as_string().0
    }
}
