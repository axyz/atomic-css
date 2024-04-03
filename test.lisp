(electron `red` (color `#ff0000`))
(electron `bg_green` (background-color `#00ff00`))
(molecule `flag`
  (atom `root` (electrons `red` `bg_green`))
  (atom `label` (import `button` `label`))
  (rule `${root}` `
    padding: 1rem;
    margin: 0;
  `)
  (@ `foo`)
  (@ `bar` `baz`)
  (@ `media` `(min-width: 1024px)`
    (rule `${root}` `padding: 1.5rem`)))

