use crate::traits::pdf_represent::PdfRepresentatation;
#[allow(dead_code)]
pub struct PdfPage {
    pub parent: (i32, i32),
    pub media_box: (i32, i32, i32, i32),
    pub crop_box: (i32, i32, i32, i32),
    pub rotate: i32,
    pub user_unit: f32,
    pub contents_ref: String,
    pub resources: String,
}

impl PdfRepresentatation for PdfPage {
    fn get_as_string(&self) -> (String, u64) {
        let parent_ref = format!("{} {} R", self.parent.0, self.parent.1);
        let media_box = format!(
            "[{} {} {} {}]",
            self.media_box.0, self.media_box.1, self.media_box.2, self.media_box.3
        );
        let crop_box = format!(
            "[{} {} {} {}]",
            self.crop_box.0, self.crop_box.1, self.crop_box.2, self.crop_box.3
        );

        let s = format!(
            "<<
  /Type /Page
  /Parent {parent_ref}
  /MediaBox {media_box}
  /CropBox {crop_box}
  /Rotate {}
  /UserUnit {}
  /Contents {}
  /Resources {}
>>",
            self.rotate,
            self.user_unit,
            self.contents_ref,
            self.resources
        );

        (s.clone(), s.len() as u64)
    }


    fn get_wrapped(&self, id: u64, generation: u64) -> String {
        let (body, _) = self.get_as_string();
        format!("{id} {generation} obj\n{body}\nendobj")
    }
}
