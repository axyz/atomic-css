pub type ElectronName = String;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Electron {
    pub name: ElectronName,
    pub property: String,
    pub value: String,
}

impl Electron {
    pub fn new(name: &str, property: &str, value: &str) -> Self {
        Electron {
            name: name.to_string(),
            property: property.to_string(),
            value: value.to_string(),
        }
    }
}
