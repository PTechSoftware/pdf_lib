

pub trait PdfRepresentatation {
    fn get_as_string(&self)->(String,u64);

    fn get_wrapped(&self , id:u64 , generation : u64)-> String;
}