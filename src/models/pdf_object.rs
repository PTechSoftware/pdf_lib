use crate::traits::pdf_represent::PdfRepresentatation;

#[allow(dead_code)]
pub struct PdfObject<T>
where
    T: PdfRepresentatation,
{
    element: T,
    id: i32,
    generation: i32,
}

impl<T> PdfObject<T>
where
    T: PdfRepresentatation,
{
    #[allow(dead_code)]
    pub fn new(element: T, id: i32) -> PdfObject<T> {
        Self {
            element,
            id,
            generation: 0,
        }
    }
}

impl<T: PdfRepresentatation> PdfRepresentatation for PdfObject<T> {
    fn get_as_string(&self) -> String {
        format!(
"{} {} obj
{}
endobj",
            self.id,
            self.generation,
            self.element.get_as_string()
        )
    }
}

