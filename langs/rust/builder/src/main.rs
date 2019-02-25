fn main() {
    let p: Person = PersonBuilder::new()
        .id(123)
        .name("foo")
        .age(99)
        .address("neverland")
        .build();
    println!("{:?}", p);
}

#[derive(Debug)]
struct Person {
    id: u32,
    name: String,
    age: u32,
    address: Option<String>,
    zipcode: Option<String>,
}

struct PersonBuilder<IdType, NameType, AgeType> {
    id: IdType,
    name: NameType,
    age: AgeType,
    address: Option<String>,
    zipcode: Option<String>,
}

impl PersonBuilder<(), (), ()> {
    pub fn new() -> Self {
        Self {
            id: (),
            name: (),
            age: (),
            address: None,
            zipcode: None,
        }
    }
}

impl PersonBuilder<u32, String, u32> {
    pub fn build(self) -> Person {
        Person {
            id: self.id,
            name: self.name,
            age: self.age,
            address: self.address,
            zipcode: self.zipcode,
        }
    }
}

impl<IdType, NameType, AgeType> PersonBuilder<IdType, NameType, AgeType> {
    pub fn id(self, id: u32) -> PersonBuilder<u32, NameType, AgeType> {
        PersonBuilder {
            id: id,
            name: self.name,
            age: self.age,
            address: self.address,
            zipcode: self.zipcode,
        }
    }
    pub fn name<S: Into<String>>(self, name: S) -> PersonBuilder<IdType, String, AgeType> {
        PersonBuilder {
            id: self.id,
            name: name.into(),
            age: self.age,
            address: self.address,
            zipcode: self.zipcode,
        }
    }
    pub fn age(self, age: u32) -> PersonBuilder<IdType, NameType, u32> {
        PersonBuilder {
            id: self.id,
            name: self.name,
            age: age,
            address: self.address,
            zipcode: self.zipcode,
        }
    }
    pub fn address<S: Into<String>>(mut self, address: S) -> Self {
        self.address = Some(address.into());
        self
    }

    pub fn zipcode<S: Into<String>>(mut self, zipcode: S) -> Self {
        self.zipcode = Some(zipcode.into());
        self
    }
}
