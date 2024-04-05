use petgraph::algo::toposort;
use petgraph::algo::Cycle;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet};

use crate::atom::*;
use crate::electron::*;
use crate::molecule::*;

#[derive(Clone, Debug, Default)]
struct Dependencies {
    node_ids: HashMap<MoleculeName, NodeIndex>,
    graph: DiGraph<MoleculeName, ()>,
}

impl Dependencies {
    fn contains(&self, molecule: &MoleculeName) -> bool {
        self.node_ids.contains_key(molecule)
    }

    fn add_molecule(&mut self, molecule: MoleculeName) -> NodeIndex {
        if !self.contains(&molecule.to_owned()) {
            let molecule_id = self.graph.add_node(molecule.to_owned());
            self.node_ids.insert(molecule, molecule_id);
            molecule_id
        } else {
            self.node_ids.get(&molecule).unwrap().to_owned()
        }
    }

    fn add_dependency(&mut self, from: MoleculeName, to: MoleculeName) {
        let to_id = if let Some(id) = self.node_ids.get(&from) {
            *id
        } else {
            self.add_molecule(from)
        };

        let from_id = if let Some(id) = self.node_ids.get(&to) {
            *id
        } else {
            self.add_molecule(to)
        };

        self.graph
            .add_edge(from_id.to_owned(), to_id.to_owned(), ());
    }

    fn get_topological_order(&self) -> Result<Vec<MoleculeName>, Cycle<NodeIndex>> {
        let mut result = Vec::new();
        let nodes = toposort(&self.graph, None)?;

        for node in &nodes {
            result.push(self.graph[*node].to_owned());
        }

        Ok(result)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Organism {
    pub electrons: HashMap<ElectronName, Electron>,
    pub molecules: HashMap<MoleculeName, Molecule>,
    dependencies: Dependencies,
    exports: HashMap<MoleculeName, HashMap<AtomName, Vec<String>>>,
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
            .insert(electron.name.to_owned(), electron.to_owned());
        self.css.push_str(&format!(
            ".{} {{ {}: {} }}",
            electron.name, electron.property, electron.value
        ));
    }

    pub fn insert_molecule(&mut self, molecule: &Molecule) {
        self.molecules
            .insert(molecule.name.to_owned(), molecule.to_owned());
        self.css.push_str(&molecule.get_css());
        self.dependencies.add_molecule(molecule.name.clone());

        for atom in molecule.atoms.values() {
            for (molecule_name, _) in &atom.imports {
                self.dependencies
                    .add_dependency(molecule.name.to_owned(), molecule_name.to_owned());
            }
        }
    }

    fn get_molecule_exports(&self, molecule: MoleculeName) -> HashMap<AtomName, Vec<String>> {
        let mut exports = HashMap::new();

        let Some(molecule) = self.molecules.get(&molecule) else {
            return exports;
        };

        for atom_name in molecule.atoms.keys() {
            let mut atom_classes = HashSet::new();

            if let Some(selector) = molecule.get_atom_selector(atom_name) {
                if molecule.has_hashable_content(atom_name) {
                    atom_classes.insert(&selector[1..]);
                }
            }

            if let Some(electrons) = molecule.get_atom_electrons(atom_name) {
                for electron in electrons {
                    // TODO: electron hasing
                    atom_classes.insert(electron);
                }
            }

            let Some(imports) = molecule.get_atom_imports(atom_name) else {
                continue;
            };

            for (molecule_name, atom_name) in imports {
                // for this to exist this function mus tbe called respecting the
                // topological order of the dependencies
                let Some(molecule_exports) = self.exports.get(molecule_name) else {
                    continue;
                };
                if let Some(classes) = molecule_exports.get(atom_name) {
                    for class in classes {
                        atom_classes.insert(class);
                    }
                }
            }

            exports.insert(
                atom_name.to_string(),
                atom_classes.iter().map(|class| class.to_string()).collect(),
            );
        }

        exports
    }

    pub fn update_exports(&mut self) -> Result<(), Cycle<NodeIndex>> {
        let molecules = self.dependencies.get_topological_order()?;
        for molecule_name in &molecules {
            self.exports.insert(
                molecule_name.to_string(),
                self.get_molecule_exports(molecule_name.to_owned()),
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn it_works() {
        let electrons = vec![
            electron!("red", "color", "#ff0000"),
            electron!("bg_green", "background-color", "#00ff00"),
        ];

        let flag = molecule!("flag")
            .with_atom(atom!("root").with_electrons(vec!["bg_green"]))
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

        let mut library = organism!()
            .with_electrons(electrons)
            .with_molecules(vec![flag, button]);

        library.update_exports().expect("Failed to update exports");

        print!(">>> {:?}", &library);

        for molecule in library.molecules.values() {
            println!("{}: {}", molecule.name, molecule.get_css());
        }
    }
}
