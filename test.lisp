(electron `red` (color `#ff0000`))
(electron `blue` (color `#0000ff`))
(electron `bg_green` (background-color `#00ff00`))

(molecule `button`
  (atom `label` (electrons `blue`)))

(molecule `flag`
  (atom `root` (electrons `red` `bg_green`))
  (atom `label` (import `button` `label`))
  (& `${root}`
    (padding `1rem`)
    (& `&:hover`
      (background-color `#ff00ff`))
    (margin `0`)
    (@ `media` `print` (display `none`)))
  (@ `foo`)
  (@ `bar` `baz`)
  (@ `media` `(min-width: 1024px)`
    (& `${root}` (padding `1.5rem`))))

