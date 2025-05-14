
#[allow(dead_code)]
pub trait PdfRepresentatation {
    fn get_as_string(&self)->(String,u64);

    fn get_wrapped(&self , id:u64 , generation : u64)-> String;
    fn get_wrapped_bytes(&self, id: u64, generation: u64) -> Vec<u8> {
        self.get_wrapped(id, generation).into_bytes()
    }
}