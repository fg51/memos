use std::marker::PhantomData;

fn main() {
    let p: Person = PersonBuilder::new()
        .id(123)
        .name("foo")
        .age(99)
        .address("neverland")
        .build();
    println!("{:?}", p);
}

#[allow(dead_code)]
struct Asigned;

#[allow(dead_code)]
struct NotAsigned;

trait ToAsign {}
impl ToAsign for Asigned {}
impl ToAsign for NotAsigned {}

#[derive(Debug)]
struct Person {
    id: u32,
    name: String,
    age: u32,
    address: Option<String>,
    zipcode: Option<String>,
}

struct PersonBuilder<IdType, NameType, AgeType>
where
    IdType: ToAsign,
    NameType: ToAsign,
    AgeType: ToAsign,
{
    _marker_id: PhantomData<fn() -> IdType>,
    _marker_name: PhantomData<fn() -> NameType>,
    _marker_age: PhantomData<fn() -> AgeType>,

    id: u32,
    name: String,
    age: u32,
    address: Option<String>,
    zipcode: Option<String>,
}

impl PersonBuilder<NotAsigned, NotAsigned, NotAsigned> {
    pub fn new() -> PersonBuilder<NotAsigned, NotAsigned, NotAsigned> {
        PersonBuilder {
            _marker_id: PhantomData,
            _marker_name: PhantomData,
            _marker_age: PhantomData,

            id: 0,
            name: "".to_owned(),
            age: 0,
            address: None,
            zipcode: None,
        }
    }
}

impl PersonBuilder<Asigned, Asigned, Asigned> {
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

impl<IdType, NameType, AgeType> PersonBuilder<IdType, NameType, AgeType>
where
    IdType: ToAsign,
    NameType: ToAsign,
    AgeType: ToAsign,
{
    pub fn id(self, id: u32) -> PersonBuilder<Asigned, NameType, AgeType> {
        PersonBuilder {
            _marker_id: PhantomData,
            _marker_name: PhantomData,
            _marker_age: PhantomData,

            id: id,
            name: self.name,
            age: self.age,
            address: self.address,
            zipcode: self.zipcode,
        }
    }

    pub fn name<S: Into<String>>(self, name: S) -> PersonBuilder<IdType, Asigned, AgeType> {
        PersonBuilder {
            _marker_id: PhantomData,
            _marker_name: PhantomData,
            _marker_age: PhantomData,

            id: self.id,
            name: name.into(),
            age: self.age,
            address: self.address,
            zipcode: self.zipcode,
        }
    }

    pub fn age(self, age: u32) -> PersonBuilder<IdType, NameType, Asigned> {
        PersonBuilder {
            _marker_id: PhantomData,
            _marker_name: PhantomData,
            _marker_age: PhantomData,

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

    #[allow(dead_code)]
    pub fn zipcode<S: Into<String>>(mut self, zipcode: S) -> Self {
        self.zipcode = Some(zipcode.into());
        self
    }
}
