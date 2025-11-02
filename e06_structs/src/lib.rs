#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8
}

#[derive(Debug, Default)]
pub struct Book {
    pub title: String,
    pub pages: u32,
    pub publication_date: u64,
    pub is_available: bool
}

impl Book {
    pub fn new(title: String, is_available: bool) -> BookBuilder {
        BookBuilder {
            title,
            is_available,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct BookBuilder {
    pub title: String,
    pub pages: Option<u32>,
    pub publication_date: Option<u64>,
    pub is_available: bool
}

impl BookBuilder {
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn pages(mut self, pages: u32) -> Self {
        self.pages = Some(pages);
        self
    }

    pub fn publication_date(mut self, publication_date: u64) -> Self {
        self.publication_date = Some(publication_date);
        self
    }

    pub fn is_available(mut self, is_available: bool) -> Self {
        self.is_available = is_available;
        self
    }
    
    pub fn build(self) -> Book {
        Book { 
            title: self.title, 
            pages: self.pages.unwrap_or_default(), 
            publication_date: self.publication_date.unwrap_or_default(), 
            is_available: self.is_available }
    }
}

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: u8,
}