use std::collections::HashMap;

use crate::electron::*;
use crate::molecule::*;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Organism {
    pub electrons: HashMap<ElectronName, Electron>,
    pub molecules: HashMap<MoleculeName, Molecule>,
    css: String,
}

impl Organism {
    pub fn new() -> Self {
        Organism {
            ..Default::default()
        }
    }

    pub fn with_electron(mut self, electron: Electron) -> Self {
        self.insert_electron(&electron);
        self
    }

    pub fn with_molecule(mut self, molecule: Molecule) -> Self {
        self.insert_molecule(&molecule);
        self
    }

    pub fn with_electrons(mut self, electrons: Vec<Electron>) -> Self {
        for electron in electrons {
            self.insert_electron(&electron);
        }
        self
    }

    pub fn with_molecules(mut self, molecules: Vec<Molecule>) -> Self {
        for molecule in molecules {
            self.insert_molecule(&molecule);
        }
        self
    }

    pub fn insert_electron(&mut self, electron: &Electron) {
        self.electrons
            .insert(electron.name.clone(), electron.clone());
        self.css.push_str(&format!(
            ".{} {{ {}: {} }}",
            electron.name, electron.property, electron.value
        ));
    }

    pub fn insert_molecule(&mut self, molecule: &Molecule) {
        self.molecules
            .insert(molecule.name.clone(), molecule.clone());
        self.css.push_str(&molecule.get_css())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atom::*;
    use crate::*;

    #[test]
    fn it_works() {
        let electrons = vec![
            electron!("red", "color", "#ff0000"),
            electron!("bg_green", "background-color", "#00ff00"),
        ];

        let flag = molecule!("flag")
            .with_atom(atom!("root"))
            .with_atom(atom!("label").with_imports(vec![("button", "label")]))
            .with_css_rule(rule!("${root}", "padding: 1rem"))
            .with_css_at_rule(at_rule!("foo"))
            .with_css_at_rule(at_rule!("bar", "baz"))
            .with_css_at_rule(at_rule!(
                "media",
                "(min-width: 1024px)",
                vec![rule!("${root}", "padding: 1.5rem")]
            ));

        let button = molecule!("button").with_atom(atom!("label").with_electrons(vec!["red"]));

        let library = organism!()
            .with_electrons(electrons)
            .with_molecules(vec![flag, button]);

        print!(">>> {:?}", &library);

        for molecule in library.molecules.values() {
            println!("{}: {}", molecule.name, molecule.get_css());
        }
    }
}
