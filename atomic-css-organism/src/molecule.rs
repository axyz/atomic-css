use derive_more::Display;
use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::atom::*;
use crate::css::*;
use crate::electron::*;

fn template_string(template: &str, values: &HashMap<String, String>) -> String {
    let re = Regex::new(r"\$\{([^}]+)\}").unwrap();
    let mut result = template.to_string();
    for cap in re.captures_iter(template) {
        let key = cap.get(1).unwrap().as_str();
        let value = values.get(key).unwrap();
        result = result.replace(&format!("${{{}}}", key), value);
    }
    result
}

fn get_variables(str: &str) -> Vec<String> {
    let re = Regex::new(r"\$\{([^}]+)\}").unwrap();
    let mut variables = Vec::new();
    for cap in re.captures_iter(str) {
        variables.push(cap.get(1).unwrap().as_str().to_string());
    }
    variables
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct HashedAtoms {
    molecule_name: MoleculeName,
    selectors: HashMap<AtomName, String>,
    hashable_contents: HashMap<AtomName, String>,
}

impl HashedAtoms {
    fn new(molecule_name: &str) -> Self {
        HashedAtoms {
            molecule_name: molecule_name.to_string(),
            ..Default::default()
        }
    }

    fn update_atom_selector(&mut self, atom: &Atom) {
        self.selectors.insert(
            atom.name.clone(),
            format!(
                ".{}_{}_{}",
                self.molecule_name,
                atom.name,
                // TODO: use proper hash instead of name_len
                // also support a debug mode where the name remains readable
                self.hashable_contents
                    .get(&atom.name)
                    .expect("ERROR: usage of non existent atom")
                    .len()
            ),
        );
    }

    fn update_atom_hashable_contents(&mut self, atom: &Atom, content: &str) {
        let previous_content = match self.hashable_contents.get(&atom.name) {
            Some(c) => c,
            None => "",
        };
        let next_content = format!("{}{}", previous_content, content);
        self.hashable_contents
            .insert(atom.name.clone(), next_content);
        self.update_atom_selector(atom);
    }
}

pub type MoleculeName = String;

#[derive(Clone, Eq, PartialEq, Debug, Default, Display)]
#[display(fmt = "#Molecule({})", name)]
pub struct Molecule {
    pub name: MoleculeName,
    dependencies: HashSet<MoleculeName>,
    pub atoms: HashMap<AtomName, Atom>,
    hashed_atoms: HashedAtoms,
    pub css: String,
}

impl Molecule {
    pub fn new(name: &str) -> Self {
        Molecule {
            name: name.to_string(),
            hashed_atoms: HashedAtoms::new(name),
            ..Default::default()
        }
    }

    pub fn with_atom(mut self, atom: Atom) -> Self {
        self.insert_atom(&atom);
        self
    }

    pub fn with_css_rule(mut self, css_rule: CSSRule) -> Self {
        self.insert_css_rule(&css_rule);
        self
    }

    pub fn with_css_at_rule(mut self, css_at_rule: CSSAtRule) -> Self {
        self.insert_css_at_rule(&css_at_rule);
        self
    }

    pub fn with_atoms(mut self, atoms: Vec<Atom>) -> Self {
        for atom in atoms {
            self.insert_atom(&atom);
        }
        self
    }

    pub fn with_css_rules(mut self, css_rules: Vec<CSSRule>) -> Self {
        for css_rule in css_rules {
            self.insert_css_rule(&css_rule);
        }
        self
    }

    pub fn with_css_at_rules(mut self, css_at_rules: Vec<CSSAtRule>) -> Self {
        for css_at_rule in css_at_rules {
            self.insert_css_at_rule(&css_at_rule);
        }
        self
    }

    pub fn insert_atom(&mut self, atom: &Atom) {
        self.atoms.insert(atom.name.clone(), atom.clone());
        for (molecule, _) in &atom.imports {
            self.dependencies.insert(molecule.clone());
        }
        self.hashed_atoms.update_atom_hashable_contents(atom, "");
        self.hashed_atoms.update_atom_selector(atom);
    }

    fn update_hashable_contents_from_css(&mut self, css: &str) {
        let variables = get_variables(css);
        for variable in variables {
            if let Some(atom) = self.atoms.get(&variable) {
                self.hashed_atoms.update_atom_hashable_contents(atom, css);
            }
        }
    }

    pub fn insert_css_rule(&mut self, css_rule: &CSSRule) {
        self.css.push_str(&css_rule.get_css());

        self.update_hashable_contents_from_css(&css_rule.get_css());
    }

    pub fn get_css(&self) -> String {
        template_string(&self.css, &self.hashed_atoms.selectors)
    }

    pub fn get_atom_selector(&self, atom_name: &str) -> Option<&String> {
        self.hashed_atoms.selectors.get(atom_name)
    }

    pub fn get_atom_imports(&self, atom_name: &str) -> Option<&Vec<(MoleculeName, AtomName)>> {
        self.atoms.get(atom_name).map(|atom| &atom.imports)
    }

    pub fn get_atom_electrons(&self, atom_name: &str) -> Option<&Vec<ElectronName>> {
        self.atoms.get(atom_name).map(|atom| &atom.electrons)
    }

    pub fn has_hashable_content(&self, atom_name: &str) -> bool {
        self.hashed_atoms.hashable_contents.contains_key(atom_name)
            && !self.hashed_atoms.hashable_contents[atom_name].is_empty()
    }

    pub fn insert_css_at_rule(&mut self, css_at_rule: &CSSAtRule) {
        self.css.push_str(&css_at_rule.get_css());

        self.update_hashable_contents_from_css(&css_at_rule.get_css());
    }
}
