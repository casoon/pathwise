# pathwise

Rust-Crate: generische Bausteine für Search-, Optimization- und
Constraint-Algorithmen hinter einer einheitlichen, Trait-basierten API.
Konzept & Herkunft: `README.md`. Umsetzungsplan: `plan/`.

Projektname (Repo) und voraussichtlicher crates.io-Paketname sind
identisch, `pathwise` — vor Veröffentlichung Verfügbarkeit erneut prüfen.

## Positionierung (siehe `plan/01-concept.md` für Details)

Kein Konkurrent zu `TheAlgorithms/Rust` (Lern-/Enzyklopädie-Sammlung) oder
`EbTech/rust-algorithms` (Competitive-Programming-Cookbook). Auch kein
Ersatz für `petgraph` — eher perspektivisch dessen Ergänzung/Integration
für Graph-Primitive.

Stattdessen: eine kleine, streng kuratierte Menge an Search-/
Optimization-/Constraint-Algorithmen mit einer gemeinsamen generischen
API (`Problem`-Trait), Tests, Benchmarks und dokumentierten Trade-offs.
Nichts implementieren, was `std` oder ein etabliertes Crate bereits gut
löst.

## Architektur (Arbeitstitel, siehe `plan/01-concept.md` für Details)

```
core (Problem-Trait: State, Move, Cost, Score)
 ├── search (bfs, dfs, iddfs, ucs, best_first, astar, beam_search)
 └── optimization (branch_and_bound, hill_climbing, local_search,
       simulated_annealing)
constraint (backtracking, forward_checking, propagation, AC-3, MRV, LCV)
graph (topological_sort; matching/flow/coloring später)
```

Langfristige Stoßrichtung (nicht Teil von 0.1, nur Kontext): `pathwise`
ist die unterste Schicht eines geplanten Stapels
`pathwise → constraint solver → scheduling framework → Stundenplanung`.
Nicht vorauseilend implementieren — jede Schicht ist ein eigenständig
nutzbares Zwischenprodukt.

## Arbeitsweise

- Aktueller Stand & nächster Schritt: `plan/00-STATUS.md`.
- Konzept & Scope-Entscheidungen: `plan/01-concept.md`.
- Getroffene Entscheidungen: `plan/DECISIONS.md` — dort nachschlagen,
  bevor offene Fragen neu aufgerollt werden.

## Feste Regeln

- Lizenz: **MIT**, von Anfang an (`Cargo.toml`: `license = "MIT"`).
- Kein `unsafe` ohne expliziten Grund und Kommentar.
- Nichts implementieren, wofür `std` oder ein etabliertes Crate bereits
  eine gute Lösung bietet (siehe Scope-Prinzip in `plan/01-concept.md`).
- Jede veröffentlichte Funktion dokumentiert Komplexität, Voraussetzungen
  (z. B. zulässige Heuristik bei A*) und Referenz (Originalpaper/CLRS),
  nicht nur eine Kurzbeschreibung.

## Definition of Done

Noch nicht definiert — Konzeptphase. Wird pro Phase in `plan/0N-*.md`
festgelegt, sobald mit der Implementierung begonnen wird.
