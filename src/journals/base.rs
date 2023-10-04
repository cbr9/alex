pub trait Journal {
    fn title(&self) -> Option<String>;
    fn download_pdf(&self) -> Option<File>;
}
