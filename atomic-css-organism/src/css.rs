use derive_more::Display;

#[derive(Clone, Eq, PartialEq, Debug, Default, Display)]
#[display(fmt = "#CSSDeclaration({}: {})", property, value)]
pub struct CSSDeclaration {
    pub property: String,
    pub value: String,
}

impl CSSDeclaration {
    pub fn new(property: &str, value: &str) -> Self {
        Self {
            property: property.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Display)]
pub enum CSSNode {
    CSSRule(CSSRule),
    CSSAtRule(CSSAtRule),
    CSSDeclaration(CSSDeclaration),
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Display)]
#[display(fmt = "#CSSRule({} {{ ... }})", selector)]
pub struct CSSRule {
    pub selector: String,
    children: Vec<CSSNode>,
}

impl CSSRule {
    pub fn new(selector: &str) -> Self {
        Self {
            selector: selector.to_string(),
            ..Default::default()
        }
    }

    pub fn get_css(&self) -> String {
        let mut css = format!("{}{{", self.selector);
        for child in &self.children {
            match child {
                CSSNode::CSSDeclaration(declaration) => {
                    css.push_str(&format!("{}:{};", declaration.property, declaration.value))
                }
                CSSNode::CSSAtRule(at_rule) => css.push_str(&at_rule.get_css()),
                CSSNode::CSSRule(rule) => css.push_str(&rule.get_css()),
            }
        }
        css.push('}');
        css
    }

    pub fn with_declaration(mut self, declaration: CSSDeclaration) -> Self {
        self.insert_declaration(&declaration);
        self
    }

    pub fn with_at_rule(mut self, at_rule: CSSAtRule) -> Self {
        self.insert_at_rule(&at_rule);
        self
    }

    pub fn with_rule(mut self, rule: CSSRule) -> Self {
        self.insert_rule(&rule);
        self
    }

    pub fn insert_declaration(&mut self, declaration: &CSSDeclaration) {
        self.children
            .push(CSSNode::CSSDeclaration(declaration.clone()));
    }

    pub fn insert_at_rule(&mut self, at_rule: &CSSAtRule) {
        self.children.push(CSSNode::CSSAtRule(at_rule.clone()));
    }

    pub fn insert_rule(&mut self, rule: &CSSRule) {
        self.children.push(CSSNode::CSSRule(rule.clone()));
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Display)]
#[display(fmt = "#CSSAtRule(@{} {:?} {{ ... }})", name, params)]
pub struct CSSAtRule {
    pub name: String,
    pub params: Option<String>,
    children: Vec<CSSNode>,
}

impl CSSAtRule {
    pub fn new(name: &str, params: Option<&str>) -> Self {
        let mut at_rule = Self {
            ..Default::default()
        };
        at_rule.name = name.to_string();
        if let Some(par) = params {
            at_rule.params = Some(par.to_string());
        }
        at_rule
    }

    pub fn with_rule(mut self, rule: CSSRule) -> Self {
        self.insert_rule(&rule);
        self
    }

    pub fn with_at_rule(mut self, at_rule: CSSAtRule) -> Self {
        self.insert_at_rule(&at_rule);
        self
    }

    pub fn with_declaration(mut self, declaration: CSSDeclaration) -> Self {
        self.insert_declaration(&declaration);
        self
    }

    pub fn insert_rule(&mut self, rule: &CSSRule) {
        self.children.push(CSSNode::CSSRule(rule.clone()));
    }

    pub fn insert_at_rule(&mut self, at_rule: &CSSAtRule) {
        self.children.push(CSSNode::CSSAtRule(at_rule.clone()));
    }

    pub fn insert_declaration(&mut self, declaration: &CSSDeclaration) {
        self.children
            .push(CSSNode::CSSDeclaration(declaration.clone()));
    }

    pub fn get_css(&self) -> String {
        let mut css = format!("@{}", self.name);
        if let Some(params) = &self.params {
            css.push_str(&format!(" {}", params));
        }

        if !self.children.is_empty() {
            css.push('{');
            for child in &self.children {
                match child {
                    CSSNode::CSSRule(rule) => css.push_str(&rule.get_css()),
                    CSSNode::CSSAtRule(at_rule) => css.push_str(&at_rule.get_css()),
                    CSSNode::CSSDeclaration(decl) => {
                        css.push_str(&format!("{}:{};", decl.property, decl.value))
                    }
                }
            }
            css.push('}');
        } else {
            css.push(';');
        }

        css
    }
}
