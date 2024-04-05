use atomic_css_organism::atom::*;
use atomic_css_organism::electron::*;
use atomic_css_organism::molecule::*;
use atomic_css_organism::organism::*;
use atomic_css_parser::parser::*;

type Error = (String, Vec<Node>);

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
    pub fn run(&mut self, src: &str) -> Result<(), Error> {
        let ast = parse(src).expect("Failed to parse file");
        for node in &ast {
            if let Node::Function(name, args) = node {
                call_organism_function(name, args, &mut self.organism)?;
            }
        }

        Ok(())
    }
}

fn call_organism_function(name: &str, args: &[Node], organism: &mut Organism) -> Result<(), Error> {
    match name {
        "electron" => Ok(handle_electron(organism, args)?),
        "molecule" => Ok(handle_molecule(organism, args)?),
        _ => Ok(()),
    }
}

fn call_molecule_function(name: &str, args: &[Node], molecule: &mut Molecule) -> Result<(), Error> {
    match name {
        "atom" => Ok(handle_atom(molecule, args)?),
        "rule" => Ok(handle_rule(molecule, args)?),
        "@" => Ok(handle_at_rule(molecule, args)?),
        _ => Ok(()),
    }
}

fn call_atom_function(name: &str, args: &[Node], atom: &mut Atom) -> Result<(), Error> {
    match name {
        "electrons" => Ok(handle_electrons(atom, args)?),
        "import" => Ok(handle_import(atom, args)?),
        _ => Ok(()),
    }
}

fn handle_electron(organism: &mut Organism, args: &[Node]) -> Result<(), Error> {
    match &args {
        [Node::String(name), Node::Function(property, val)] => {
            let value = if let [Node::String(value)] = &val[..1] {
                value
            } else {
                return Err(("Expected string value".to_owned(), args.to_vec()));
            };
            organism.insert_electron(&Electron::new(name, property, value));
            Ok(())
        }
        _ => Err(("Invalid electron".to_owned(), args.to_vec())),
    }
}

fn handle_electrons(atom: &mut Atom, args: &[Node]) -> Result<(), Error> {
    for node in args {
        match node {
            Node::String(name) => atom.insert_electron(name),
            _ => return Err(("Invalid electron".to_owned(), args.to_vec())),
        }
    }
    Ok(())
}

fn handle_import(atom: &mut Atom, args: &[Node]) -> Result<(), Error> {
    match &args {
        [Node::String(molecule), Node::String(imported_atom)] => {
            atom.insert_import(molecule, imported_atom);
            Ok(())
        }
        _ => Err(("Invalid import".to_owned(), args.to_vec())),
    }
}

fn handle_atom(molecule: &mut Molecule, args: &[Node]) -> Result<(), Error> {
    match &args[0] {
        Node::String(name) => {
            let mut atom = Atom::new(name);

            for node in &args[1..] {
                if let Node::Function(name, args) = node {
                    call_atom_function(name, args, &mut atom)?;
                }
            }

            molecule.insert_atom(&atom);
            Ok(())
        }
        _ => Err(("Invalid atom".to_owned(), args.to_vec())),
    }
}

fn handle_molecule(organism: &mut Organism, args: &[Node]) -> Result<(), Error> {
    match &args[0] {
        Node::String(name) => {
            let mut molecule = Molecule::new(name);

            for node in &args[1..] {
                if let Node::Function(name, args) = node {
                    call_molecule_function(name, args, &mut molecule)?;
                }
            }

            organism.insert_molecule(&molecule);
            Ok(())
        }
        _ => Err(("Invalid molecule".to_owned(), args.to_vec())),
    }
}

fn handle_rule(molecule: &mut Molecule, args: &[Node]) -> Result<(), Error> {
    match &args {
        [Node::String(selector), Node::String(css)] => {
            molecule.insert_css_rule(&CSSRule::new(selector, css), None);
            Ok(())
        }
        _ => Err(("Invalid rule".to_owned(), args.to_vec())),
    }
}

fn handle_at_rule(molecule: &mut Molecule, args: &[Node]) -> Result<(), Error> {
    match &args {
        [Node::String(name)] => {
            molecule.insert_css_at_rule(&CSSAtRule::new(name, None, None));
            Ok(())
        }
        [Node::String(name), Node::String(params)] => {
            molecule.insert_css_at_rule(&CSSAtRule::new(name, Some(params), None));
            Ok(())
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
                    _ => return Err(("Invalid rule".to_owned(), args.to_vec())),
                }
            }

            molecule.insert_css_at_rule(&css_at_rule);
            Ok(())
        }
        _ => Err(("Invalid at rule".to_owned(), args.to_vec())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
