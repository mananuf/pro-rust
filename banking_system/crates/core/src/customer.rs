pub type CustomerId = u64;

#[derive(Debug, Default)]
pub struct Customer {
    pub id: CustomerId,
    pub name: String,
}

impl Customer {
    pub fn builder(id: CustomerId) -> CustomerBuilder {
        CustomerBuilder {
            id,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct CustomerBuilder {
    pub id: CustomerId,
    pub name: Option<String>,
}

impl CustomerBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> Customer {
        Customer {
            id: self.id,
            name: self.name.unwrap_or_default(),
        }
    }
}
