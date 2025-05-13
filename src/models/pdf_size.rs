

#[allow(dead_code)]
pub enum PdfSize {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
    A10,
    Custom(i32, i32),
}

impl PdfSize {
    #[allow(dead_code)]
    pub fn get_size(&self) -> (i32, i32) {
        match self {
            PdfSize::A0 => (2384, 3370),
            PdfSize::A1 => (1684, 2384),
            PdfSize::A2 => (1191, 1684),
            PdfSize::A3 => (842, 1191),
            PdfSize::A4 => (595, 842),
            PdfSize::A5 => (420, 595),
            PdfSize::A6 => (298, 420),
            PdfSize::A7 => (210, 298),
            PdfSize::A8 => (147, 210),
            PdfSize::A9 => (105, 147),
            PdfSize::A10 => (74, 105),
            PdfSize::Custom(w, h) => (*w, *h),
        }
    }
}