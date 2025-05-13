use crate::traits::pdf_represent::PdfRepresentatation;

/// # Text Matrix
/// Representa una transformación de texto en PDF
/// Tm: a b c d e f
/// Matriz:
/// | a c e |
/// | b d f |
/// | 0 0 1 |
#[derive(Debug, Clone, Copy)]
pub struct Tm {
    pub a: i32, // escala en X
    pub b: i32, // inclinación vertical
    pub c: i32, // inclinación horizontal
    pub d: i32, // escala en Y
    pub e: i32, // traslación en X
    pub f: i32, // traslación en Y
}

impl Tm {
    #[allow(dead_code)]
    /// Crea una matriz de identidad trasladada a (x, y)
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            a: 1,
            b: 0,
            c: 0,
            d: 1,
            e: x,
            f: y,
        }
    }
    #[allow(dead_code)]
    pub fn set_x_scale(&mut self, x: i32) {
        self.a = x;
    }
    #[allow(dead_code)]
    pub fn set_y_scale(&mut self, y: i32) {
        self.d = y;
    }
    #[allow(dead_code)]
    pub fn set_x_offset(&mut self, x: i32) {
        self.e = x;
    }
    #[allow(dead_code)]
    pub fn set_y_offset(&mut self, y: i32) {
        self.f = y;
    }
    #[allow(dead_code)]
    pub fn set_inclination_x(&mut self, incl: i32) {
        self.c = incl;
    }
    #[allow(dead_code)]
    pub fn set_inclination_y(&mut self, incl: i32) {
        self.b = incl;
    }
    #[allow(dead_code)]
    /// Retorna la coordenada X (traslación)
    pub fn get_x(&self) -> i32 {
        self.e
    }
    #[allow(dead_code)]
    /// Retorna la coordenada Y (traslación)
    pub fn get_y(&self) -> i32 {
        self.f
    }
    #[allow(dead_code)]
    /// Retorna la matriz como string PDF
    pub fn to_pdf(&self) -> String {
        format!("{} {} {} {} {} {} Tm", self.a, self.b, self.c, self.d, self.e, self.f)
    }
}

impl PdfRepresentatation for Tm {
    fn get_as_string(&self) -> (String, u64) {
        let s = self.to_pdf();
        let size = s.len() as u64;
        (s, size)
    }
    #[allow(dead_code, unused)]
    fn get_wrapped(&self, id: u64, generation: u64) -> String {
        self.get_as_string().0
    }
}
