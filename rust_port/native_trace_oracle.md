# Native Trace Oracle

Cycles 280-289 require C-generated traces for each native engine category.

Current trace categories:

- land, tide, weather, terrain, and food
- population, movement, body, energy, and drives
- braincode, probes, social hooks, and speech
- social, episodic, territory, and family relationships
- lifecycle, pregnancy, birth, carrying, suckling, immune, and death
- save/load-visible state

The Rust trace formatter must match C field order, widths, signedness, and
sampling points before a trace category can be promoted into the absolute gate.
