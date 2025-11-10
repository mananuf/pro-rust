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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn build(self) -> Customer {
        Customer {
            id: self.id,
            name: self.name.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::customer::Customer;

    #[test]
    fn test_customer_will_be_created_successfully() {
        let customer = Customer::builder(1).build();

        assert_eq!(customer.name, "");
        assert_eq!(customer.id, 1);

        let new_customer = Customer::builder(2).name("name").build();

        assert_eq!(new_customer.name, "name");
        assert_eq!(new_customer.id, 2);
    }
}
