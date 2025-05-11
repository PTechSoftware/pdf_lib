


pub enum PageSize {
    A4,
    A3,
    Letter
}



impl PageSize {
    pub fn page_size(&self) -> (f64, f64) {
        match self {
            PageSize::A4 => {(210.00,297.00)}
            PageSize::A3 => {(297.00,420.00)}
            PageSize::Letter => {(21.59, 27.94)}
        }
        
    }
}