# Social Behavior Audit

## C Sources

The deeper social pass was mapped from:

```text
entity/social.c
entity/entity.h
entity/entity_internal.h
universe/loop.c
universe/universe.h
```

## Ported Coverage

Rust now carries native-shaped coverage for:

```text
social_meet feature observation
feature-set save/load for social classifications
pheromone, pigmentation, hair, height, and frame attraction
familiarity-driven friend/foe maintenance
probabilistic grooming with body focus, groomed markers, parasite removal, and touch pathogens
squabble/show-force/attack outcomes with disrespect, wounds, honor, fleeing, and episodic records
mate-seeking attraction and pair-bond goal updates
chat/anecdote transfer with preference-based mutation hooks
food-specific CLI episodic descriptions
```

## Remaining Gaps

This is still not a byte-for-byte port of the C social layer. The remaining
work is deeper territory naming, full family relationship inference, braincode
initialization for social graph entries, pregnancy/conception parity after
pair bonding, complete preference learning feedback, and C fixture comparison
for long social runs.
