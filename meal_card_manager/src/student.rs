use crate::types::MealCard;

#[derive(Debug, Default)]
pub struct Student {
    pub name: String,
    pub level: u32,
    pub status: StudentStatus,
    pub meal_card: MealCard,
}

impl Student {
    pub fn builder(name: &str, level: u32) -> StudentBuilder {
        StudentBuilder {
            name: name.to_string(),
            level,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum StudentStatus {
    #[default]
    Active,
    Suspended(String),
    Graduated(String),
}

#[derive(Debug, Default)]
pub struct StudentBuilder {
    pub name: String,
    pub level: u32,
    pub status: Option<StudentStatus>,
    pub meal_card: Option<MealCard>,
}

impl StudentBuilder {
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }

    pub fn status(mut self, status: StudentStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn meal_card(mut self, meal_card: MealCard) -> Self {
        self.meal_card = Some(meal_card);
        self
    }

    pub fn build(self) -> Student {
        Student {
            name: self.name,
            level: self.level,
            status: self.status.unwrap_or_default(),
            meal_card: self.meal_card.unwrap_or_default(),
        }
    }
}
