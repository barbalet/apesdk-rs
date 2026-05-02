# Territory, Family, And Lifecycle Audit

## Cycle 145-160 Coverage

Rust now keeps `simulated_iplace` territory memory in `BeingSummary`, projects
it through the C-shaped `simulated_being.events.territory` layout, round-trips
it through JSON transfer, and includes non-empty entries in the native transfer
text writer/reader with `terri{tridx=...;trnam=...;trfam=...;};`.

Per-minute movement updates the territory focus from ape-space location,
increments familiarity, rescales saturated familiarity, and assigns a stable
terrain-derived name once a place has been visited repeatedly. Chat shares a
known territory name with another being or names the current terrain if neither
being has a name yet.

Family and pregnancy state now has executable Rust hooks: mate behavior can
record conception, fetal genetics, father/mother names, child generation range,
and mate memories; lifecycle processing can create a child after gestation,
copy maternal antibodies, and create mother/father/child social relationships.
Preference learning now changes mate traits and chat preference from actual
mate/chat events.

## Parity Risk

This is native-shaped behavioral coverage, not final C trace proof. The largest
remaining parity risks are exact C child-genetics crossover, the full family
tree relationship fan-out for grandparents/siblings, and C's post-birth
carrying/suckling/weaning energy details over long seeded runs.

