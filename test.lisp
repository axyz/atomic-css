(electron `red` (color `#ff0000`))
(electron `blue` (color `#0000ff`))
(electron `bg_green` (background-color `#00ff00`))

(molecule `button`
  (atom `label` (electrons `blue`)))

(def world `foo`)

(def foo (electron `black` (color `#000000`)))

(dbg `hello` world (fn) foo)
(log `hello ` world ` ` (fn) ` ` foo)

(molecule `flag`
  (atom `root` (electrons `red` `bg_green`))
  (atom `label` (import `button` `label`))
  (rule `${root}`
    (padding `1rem`)
    (margin `0`))
  (@ `foo`)
  (@ `bar` `baz`)
  (@ `media` `(min-width: 1024px)`
    (rule `${root}` (padding `1.5rem`))))

