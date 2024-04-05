use atomic_css_organism::atom::*;
use atomic_css_organism::electron::*;
use atomic_css_organism::molecule::*;
use atomic_css_organism::organism::*;
use atomic_css_parser::parser::*;
use derive_more::Display;
use std::collections::HashMap;

type Error = (String, Vec<Node>);

#[derive(Debug, Clone, Display)]
#[allow(dead_code)]
enum Value {
    String(String),
    Void,
    Electron(Electron),
    Molecule(Molecule),
    Atom(Atom),
    CSSRule(CSSRule),
    CSSAtRule(CSSAtRule),
    #[display(fmt = "{:?}", _0)]
    Vec(Vec<Value>),
}

#[derive(Debug, Default)]
struct Context {
    variables: HashMap<String, Value>,
}

impl Context {
    fn define_variable(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_owned(), value);
    }
}

#[derive(Debug, Default)]
pub struct Runtime {
    pub organism: Organism,
    context: Context,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            ..Default::default()
        }
    }
    pub fn run(&mut self, src: &str) -> Result<(), Error> {
        let ast = parse(src).expect("Failed to parse file");
        for node in &ast {
            if let Node::Function(name, args) = node {
                self.call_organism_function(name, args)?;
            }
        }

        Ok(())
    }

    fn call_organism_function(&mut self, name: &str, args: &[Node]) -> Result<Value, Error> {
        match name {
            "electron" => Ok(self.handle_electron(args)?),
            "molecule" => Ok(self.handle_molecule(args)?),
            "log" => Ok(self.handle_log(args)?),
            "dbg" => Ok(self.handle_dbg(args)?),
            "def" => Ok(self.handle_def(args)?),
            _ => Ok(Value::Void),
        }
    }

    fn handle_def(&mut self, args: &[Node]) -> Result<Value, Error> {
        match &args {
            [Node::Identifier(name), Node::String(text)] => {
                self.context
                    .define_variable(name, Value::String(text.to_owned()));
                Ok(Value::Void)
            }
            [Node::Identifier(name), Node::Function(function, args)] => {
                let value = self.call_organism_function(function, args)?;
                self.context.define_variable(name, value);
                Ok(Value::Void)
            }
            _ => Err(("Invalid def".to_owned(), args.to_vec())),
        }
    }

    fn get_variable(&self, name: &str) -> Result<Value, Error> {
        if let Some(value) = self.context.variables.get(name) {
            Ok(value.to_owned())
        } else {
            Err((
                "Variable not found".to_owned(),
                vec![Node::Identifier(name.to_owned())],
            ))
        }
    }

    fn handle_electron(&mut self, args: &[Node]) -> Result<Value, Error> {
        match &args {
            [Node::String(name), Node::Function(property, val)] => {
                let value = if let [Node::String(value)] = &val[..1] {
                    value
                } else {
                    return Err(("Expected string value".to_owned(), args.to_vec()));
                };
                let electron = Electron::new(name, property, value);
                self.organism.insert_electron(&electron);
                Ok(Value::Electron(electron.to_owned()))
            }
            _ => Err(("Invalid electron".to_owned(), args.to_vec())),
        }
    }

    fn call_molecule_function(
        &mut self,
        name: &str,
        args: &[Node],
        molecule: &mut Molecule,
    ) -> Result<Value, Error> {
        match name {
            "atom" => Ok(self.handle_atom(molecule, args)?),
            "rule" => Ok(self.handle_rule(molecule, args)?),
            "@" => Ok(self.handle_at_rule(molecule, args)?),
            _ => Ok(Value::Void),
        }
    }

    fn call_atom_function(
        &mut self,
        name: &str,
        args: &[Node],
        atom: &mut Atom,
    ) -> Result<Value, Error> {
        match name {
            "electrons" => Ok(self.handle_electrons(atom, args)?),
            "import" => Ok(self.handle_import(atom, args)?),
            _ => Ok(Value::Void),
        }
    }

    fn handle_log(&mut self, args: &[Node]) -> Result<Value, Error> {
        for node in args {
            match node {
                Node::Identifier(name) => {
                    let value = self.get_variable(name)?;
                    print!("{}", value);
                }
                _ => print!("{}", node),
            }
        }
        println!();
        Ok(Value::Void)
    }

    fn handle_dbg(&mut self, args: &[Node]) -> Result<Value, Error> {
        for node in args {
            match node {
                Node::Identifier(name) => {
                    let value = self.get_variable(name)?;
                    print!("{:?}", value);
                }
                _ => print!("{:?}", node),
            }
        }
        println!();
        Ok(Value::Void)
    }

    fn handle_electrons(&mut self, atom: &mut Atom, args: &[Node]) -> Result<Value, Error> {
        let mut electrons = vec![];
        for node in args {
            match node {
                Node::String(name) => {
                    atom.insert_electron(name);
                    electrons.push(name)
                }
                _ => return Err(("Invalid electron".to_owned(), args.to_vec())),
            }
        }
        Ok(Value::Vec(
            electrons
                .iter()
                .map(|e| Value::String(e.to_string()))
                .collect(),
        ))
    }

    fn handle_import(&mut self, atom: &mut Atom, args: &[Node]) -> Result<Value, Error> {
        match &args {
            [Node::String(molecule), Node::String(imported_atom)] => {
                atom.insert_import(molecule, imported_atom);
                let exports = self.organism.get_exports();
                if let Some(classes) = exports.get(molecule).and_then(|m| m.get(imported_atom)) {
                    Ok(Value::Vec(
                        classes
                            .iter()
                            .map(|c| Value::String(c.to_string()))
                            .collect(),
                    ))
                } else {
                    Err(("Import not found".to_owned(), args.to_vec()))
                }
            }
            _ => Err(("Invalid import".to_owned(), args.to_vec())),
        }
    }

    fn handle_atom(&mut self, molecule: &mut Molecule, args: &[Node]) -> Result<Value, Error> {
        match &args[0] {
            Node::String(name) => {
                let mut atom = Atom::new(name);

                for node in &args[1..] {
                    if let Node::Function(name, args) = node {
                        self.call_atom_function(name, args, &mut atom)?;
                    }
                }

                molecule.insert_atom(&atom);
                Ok(Value::Atom(atom.to_owned()))
            }
            _ => Err(("Invalid atom".to_owned(), args.to_vec())),
        }
    }

    fn handle_molecule(&mut self, args: &[Node]) -> Result<Value, Error> {
        match &args[0] {
            Node::String(name) => {
                let mut molecule = Molecule::new(name);

                for node in &args[1..] {
                    if let Node::Function(name, args) = node {
                        self.call_molecule_function(name, args, &mut molecule)?;
                    }
                }

                self.organism.insert_molecule(&molecule);
                Ok(Value::Molecule(molecule.to_owned()))
            }
            _ => Err(("Invalid molecule".to_owned(), args.to_vec())),
        }
    }

    fn handle_rule(&mut self, molecule: &mut Molecule, args: &[Node]) -> Result<Value, Error> {
        match &args {
            [Node::String(selector), Node::String(css)] => {
                let css_rule = CSSRule::new(selector, css);
                molecule.insert_css_rule(&css_rule, None);
                Ok(Value::CSSRule(css_rule.to_owned()))
            }
            _ => Err(("Invalid rule".to_owned(), args.to_vec())),
        }
    }

    fn handle_at_rule(&mut self, molecule: &mut Molecule, args: &[Node]) -> Result<Value, Error> {
        match &args {
            [Node::String(name)] => {
                let css_at_rule = CSSAtRule::new(name, None, None);
                molecule.insert_css_at_rule(&css_at_rule);
                Ok(Value::CSSAtRule(css_at_rule.to_owned()))
            }
            [Node::String(name), Node::String(params)] => {
                let css_at_rule = CSSAtRule::new(name, Some(params), None);
                molecule.insert_css_at_rule(&css_at_rule);
                Ok(Value::CSSAtRule(css_at_rule.to_owned()))
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
                Ok(Value::CSSAtRule(css_at_rule.to_owned()))
            }
            _ => Err(("Invalid at rule".to_owned(), args.to_vec())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
