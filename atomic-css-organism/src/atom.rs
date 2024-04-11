use crate::electron::*;
use crate::molecule::*;
use derive_more::Display;

pub type AtomName = String;

#[derive(Clone, Eq, PartialEq, Debug, Default, Display)]
#[display(fmt = "#Atom({})", name)]
pub struct Atom {
    pub name: AtomName,
    pub electrons: Vec<ElectronName>,
    pub imports: Vec<(MoleculeName, AtomName)>,
}

impl Atom {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn with_electrons(mut self, electrons: Vec<&str>) -> Self {
        for electron in electrons {
            self.insert_electron(electron);
        }
        self
    }

    pub fn with_imports(mut self, imports: Vec<(&str, &str)>) -> Self {
        for (molecule, atom) in imports {
            self.insert_import(molecule, atom);
        }
        self
    }

    pub fn insert_electron(&mut self, electron: &str) {
        self.electrons.push(electron.to_string());
    }

    pub fn insert_import(&mut self, molecule: &str, atom: &str) {
        self.imports.push((molecule.to_string(), atom.to_string()));
    }
}
