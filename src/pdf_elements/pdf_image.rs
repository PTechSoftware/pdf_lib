use crate::high_level::pdf_pagehandler::PdfPageHandle;
use crate::models::tm::Tm;

#[derive(Debug)]
pub struct PdfImage {
    pub name: String,         // /Im1, /Logo, etc.
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub tm: Tm,               // matriz de posición y escala
    pub filter: String,       // DCTDecode (JPG), FlateDecode (PNG), ASCIIHexDecode
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
            filter: "DCTDecode".to_string(),
            color_space: "DeviceRGB".to_string(),
            bits_per_component: 8,
        }
    }

    pub fn to_object(&self, id: u64) -> (String, u64) {
        let hex_data = self
            .data
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<String>();

        let mut out = Vec::new();
        let header = format!(
            "{id} 0 obj\n<< \
/Type /XObject\n\
/Subtype /Image\n\
/Name /{}\n\
/Width {}\n\
/Height {}\n\
/ColorSpace /{}\n\
/BitsPerComponent {}\n\
/Filter /ASCIIHexDecode\n\
/Length {} >>\nstream\n",
            self.name,
            self.width,
            self.height,
            self.color_space,
            self.bits_per_component,
            hex_data.len() + 1 // por el ~> final
        );

        out.extend_from_slice(header.as_bytes());
        out.extend_from_slice(hex_data.as_bytes());
        out.extend_from_slice(b">\nendstream\nendobj");

        let salida = String::from_utf8(out).expect("Hex data should always be valid UTF-8");
        (salida, 0)
    }


    pub fn draw(&self) -> String {
        format!("q\n{} cm\n/{} Do\nQ", self.tm.to_cm(), self.name)
    }

    pub fn push_to_page(&self, page: &mut PdfPageHandle) {
        page.add_raw(&self.draw());
        page.add_image(&self.name); // Agrega al diccionario de recursos
    }
}