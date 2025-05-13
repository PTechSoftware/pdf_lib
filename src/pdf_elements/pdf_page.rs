use crate::pdf_elements::pdf_dictionary::PdfDictionary;
use crate::traits::pdf_represent::PdfRepresentatation;
#[allow(dead_code)]
#[derive(Debug)]
pub struct PdfPage {
    pub parent: (i32, i32),
    pub media_box: (i32, i32, i32, i32),
    pub crop_box: (i32, i32, i32, i32),
    pub rotate: i32,
    pub user_unit: f32,
    pub contents_ref: Vec<String>,
    pub resources: PdfDictionary,
}
impl Default for PdfPage {
    fn default() -> Self {
        Self {
            parent: (0, 0),
            media_box: (0, 0, 595, 842),
            crop_box: (0, 0, 595, 842),
            rotate: 0,
            user_unit: 1.0,
            contents_ref: Vec::new(),
            resources: PdfDictionary::new(),
        }
    }
}

impl PdfPage {
    pub fn set_parent(&mut self, id: i32, generation: i32) {
        self.parent = (id, generation);
    }

    pub fn add_content(&mut self, content_ref: String) {
        self.contents_ref.push(content_ref);
    }

    pub fn add_font(&mut self, name: &str, object_ref: &str) {
        let font_entry = format!("<< /{} {} >>", name, object_ref);
        self.resources.add_value("Font", font_entry);
    }
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
        let content_line = match self.contents_ref.len() {
            0 => String::new(),
            1 => format!("  /Contents {}", self.contents_ref[0]),
            _ => format!("  /Contents [{}]", self.contents_ref.join(" ")),
        };
        let (resources_str, _) = self.resources.get_as_string();
        let s = format!(
            "<<
  /Type /Page
  /Parent {parent_ref}
  /MediaBox {media_box}
  /CropBox {crop_box}
  /Rotate {}
  /UserUnit {}
{content_line}
  /Resources {resources_str}
>>",
            self.rotate,
            self.user_unit
        );

        (s.clone(), s.len() as u64)
    }


    fn get_wrapped(&self, id: u64, generation: u64) -> String {
        let (body, _) = self.get_as_string();
        format!("{id} {generation} obj\n{body}\nendobj")
    }
}
