use crate::high_level::pdf_pagehandler::PdfPageHandle;
use crate::models::tm::Tm;

#[derive(Debug)]
pub struct PdfImage {
    pub name: String,         // /Im1, /Logo, etc.
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub tm: Tm,               // matriz de posición y escala
    pub filter: String,       // DCTDecode (JPG), FlateDecode (PNG)
    pub color_space: String,  // DeviceRGB o DeviceGray
    pub bits_per_component: u8,
}

impl PdfImage {
    pub fn new_jpeg(name: &str, width: u32, height: u32, data: Vec<u8>, tm: Tm) -> Self {
        Self {
            name: name.to_string(),
            width,
            height,
            data,
            tm,
            filter: "DCTDecode".to_string(),
            color_space: "DeviceRGB".to_string(),
            bits_per_component: 8,
        }
    }

    pub fn new_png_raw(name: &str, width: u32, height: u32, data: Vec<u8>, tm: Tm) -> Self {
        Self {
            name: name.to_string(),
            width,
            height,
            data,
            tm,
            filter: "FlateDecode".to_string(),
            color_space: "DeviceRGB".to_string(),
            bits_per_component: 8,
        }
    }

    pub fn to_object(&self, id: u64) -> String {
        format!(
            "{id} 0 obj\n<< \
/Type /XObject\n\
/Subtype /Image\n\
/Name /{}\n\
/Width {}\n\
/Height {}\n\
/ColorSpace /{}\n\
/BitsPerComponent {}\n\
/Filter /{}\n\
/Length {} >>\nstream\n{}\nendstream\nendobj",
            self.name,
            self.width,
            self.height,
            self.color_space,
            self.bits_per_component,
            self.filter,
            self.data.len(),
            String::from_utf8_lossy(&self.data), // ❗️Si es binario real, usar write_bin
        )
    }

    pub fn draw(&self) -> String {
        format!("q\n{} cm\n/{} Do\nQ", self.tm.to_cm(), self.name)
    }

    pub fn push_to_page(&self, page: &mut PdfPageHandle) {
        page.add_raw(&self.draw());
    }
}
