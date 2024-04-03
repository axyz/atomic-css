#[macro_export]
macro_rules! atom {
    ($name:literal) => {
        Atom::new($name)
    };
}

#[macro_export]
macro_rules! electron {
    ($name:literal, $property:literal, $value:literal) => {
        Electron::new($name, $property, $value)
    };
}

#[macro_export]
macro_rules! rule {
    ($selector:literal, $css:literal) => {
        CSSRule::new($selector, $css)
    };
}

#[macro_export]
macro_rules! at_rule {
    ($name:literal, $params:expr, $rules:expr) => {
        CSSAtRule::new($name, Some($params), Some($rules))
    };

    ($name:literal, $params:expr) => {
        CSSAtRule::new($name, Some($params), None)
    };

    ($name:literal) => {
        CSSAtRule::new($name, None, None)
    };
}

#[macro_export]
macro_rules! molecule {
    ($name:literal) => {
        Molecule::new($name)
    };
}

#[macro_export]
macro_rules! organism {
    () => {
        Organism::new()
    };
}
