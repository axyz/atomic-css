use atomic_css_organism::atom::*;
use atomic_css_organism::electron::*;
use atomic_css_organism::molecule::*;
use atomic_css_organism::organism::*;
use atomic_css_parser::parser::*;

#[derive(Debug, Default)]
pub struct Runtime {
    pub organism: Organism,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            organism: Organism::new(),
        }
    }
    pub fn run(&mut self, src: &str) {
        let ast = parse(src).expect("Failed to parse file");
        for node in &ast {
            if let Node::Function(name, args) = node {
                call_organism_function(name, args, &mut self.organism);
            }
        }
    }
}

fn call_organism_function(name: &str, args: &[Node], organism: &mut Organism) {
    match name {
        "electron" => handle_electron(organism, args),
        "molecule" => handle_molecule(organism, args),
        _ => (),
    }
}

fn call_molecule_function(name: &str, args: &[Node], molecule: &mut Molecule) {
    match name {
        "atom" => handle_atom(molecule, args),
        "rule" => handle_rule(molecule, args),
        "@" => handle_at_rule(molecule, args),
        _ => (),
    }
}

fn call_atom_function(name: &str, args: &[Node], atom: &mut Atom) {
    match name {
        "electrons" => handle_electrons(atom, args),
        "import" => handle_import(atom, args),
        _ => (),
    }
}

fn handle_electron(organism: &mut Organism, args: &[Node]) {
    match &args {
        [Node::String(name), Node::Function(property, val)] => {
            let value = if let [Node::String(value)] = &val[..1] {
                value
            } else {
                panic!("Expected string value");
            };
            organism.insert_electron(&Electron::new(name, property, value));
        }
        _ => panic!("Invalid electron"),
    }
}

fn handle_electrons(atom: &mut Atom, args: &[Node]) {
    for node in args {
        match node {
            Node::String(name) => atom.insert_electron(name),
            _ => panic!("Invalid electron"),
        }
    }
}

fn handle_import(atom: &mut Atom, args: &[Node]) {
    match &args {
        [Node::String(molecule), Node::String(imported_atom)] => {
            atom.insert_import(molecule, imported_atom);
        }
        _ => panic!("Invalid import"),
    }
}

fn handle_atom(molecule: &mut Molecule, args: &[Node]) {
    match &args[0] {
        Node::String(name) => {
            let mut atom = Atom::new(name);

            for node in &args[1..] {
                if let Node::Function(name, args) = node {
                    call_atom_function(name, args, &mut atom);
                }
            }

            molecule.insert_atom(&atom);
        }
        _ => panic!("Invalid atom"),
    }
}

fn handle_molecule(organism: &mut Organism, args: &[Node]) {
    match &args[0] {
        Node::String(name) => {
            let mut molecule = Molecule::new(name);

            for node in &args[1..] {
                if let Node::Function(name, args) = node {
                    call_molecule_function(name, args, &mut molecule);
                }
            }

            organism.insert_molecule(&molecule);
        }
        _ => panic!("Invalid molecule"),
    }
}

fn handle_rule(molecule: &mut Molecule, args: &[Node]) {
    match &args {
        [Node::String(selector), Node::String(css)] => {
            molecule.insert_css_rule(&CSSRule::new(selector, css), None);
        }
        _ => panic!("Invalid rule"),
    }
}

fn handle_at_rule(molecule: &mut Molecule, args: &[Node]) {
    match &args {
        [Node::String(name)] => {
            molecule.insert_css_at_rule(&CSSAtRule::new(name, None, None));
        }
        [Node::String(name), Node::String(params)] => {
            molecule.insert_css_at_rule(&CSSAtRule::new(name, Some(params), None));
        }
        [Node::String(name), Node::String(params), rules @ ..] => {
            let mut css_at_rule = CSSAtRule::new(name, Some(params), None);
            for rule in rules {
                match rule {
                    Node::Function(function, args) if function == "rule" => {
                        if let [Node::String(selector), Node::String(css)] = &args[..2] {
                            css_at_rule.insert_rule(&CSSRule::new(selector, css));
                        }
                    }
                    _ => panic!("Invalid rule"),
                }
            }

            molecule.insert_css_at_rule(&css_at_rule);
        }
        _ => panic!("Invalid at rule"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
