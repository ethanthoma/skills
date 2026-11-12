---
name: structural-reasoning
description: >
  Analyze and simplify software structure using complexity metrics,
  explicit state modeling, first-principles reasoning, abstraction
  analysis, and adversarial design review. Use when reviewing
  architecture, refactoring complex functions, discovering implicit
  state machines, reducing branching, or challenging a proposed
  design before implementation.
---

# Structural Reasoning

Analyze the underlying structure of a problem before optimizing its
implementation.

The objective is to reduce accidental complexity while preserving
correctness, performance, and necessary domain behavior.

Do not equate fewer lines, fewer branches, or fewer abstractions with
a better design.

## 1. Analyze structural complexity

Inspect the relevant implementation for:

- Cyclomatic complexity.
- Cognitive complexity.
- Deeply nested control flow.
- Repeated conditional expressions.
- Excessive boolean parameters.
- Functions with multiple responsibilities.
- Hidden dependencies and temporal coupling.
- Duplicated state and derived values stored independently.
- Excessive indirection between related operations.

Use existing complexity-analysis tools when available.

Prefer the repository's configured tooling. Do not introduce new
dependencies merely to obtain a complexity score.

Treat metrics as diagnostic signals, not hard acceptance thresholds.

A large exhaustive `match` can be easier to understand than a
small function with interacting boolean conditions.

Identify the source of complexity before proposing changes.

## 2. Discover implicit state machines

Look for state encoded indirectly through:

- Multiple booleans representing mutually exclusive modes.
- Optional fields whose validity depends on other fields.
- Sentinel values.
- Integer or string mode identifiers.
- Scattered transition conditions.
- Call-order requirements.
- Resource lifecycle flags.
- Error recovery paths.
- Repeated checks for combinations of fields.

Ask:

- What states can this system actually occupy?
- Which combinations are valid?
- Which transitions are permitted?
- What invariants must hold in each state?
- Which operations depend on previous operations?
- Can invalid states be represented?
- Can the compiler enforce the transition rules?

Prefer explicit state representations when they reduce ambiguity.

For example:

```rust
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected {
        session: Session,
    },
    Failed {
        error: ConnectionError,
    },
}
```

Prefer this over unrelated booleans such as:

```rust
is_connected: bool,
is_connecting: bool,
has_failed: bool,
```

Do not introduce a state-machine framework when an enum and a
small transition function are sufficient.

Keep transition logic localized.

Use assertions for invariants that cannot reasonably be enforced
statically.

## 3. Reduce the problem to irreducible constraints

Exercise abstract thinking before committing to an implementation.

Identify:

1. Required inputs and outputs.
2. Invariants that must always hold.
3. External constraints and contracts.
4. Performance and resource requirements.
5. Fundamental domain operations.
6. Assumptions introduced only by the current implementation.

Temporarily disregard existing implementation details.

Ask:

> Reduce this problem to its irreducible constraints and rebuild
> it from zero.

Derive the simplest representation that satisfies those constraints.

Compare it against the current design.

Do not rewrite functioning code merely because a clean-sheet design
looks more elegant.

Use the comparison to identify removable complexity.

## 4. Challenge the architecture

Ask:

> What would a principal architect immediately rewrite before
> even starting this?

Interpret this as a request to identify foundational design problems,
not as an appeal to authority.

Look for:

- Incorrect responsibility boundaries.
- Data models that permit invalid states.
- Abstractions that obscure the actual computation.
- Unnecessary synchronization or shared mutable state.
- Expensive operations imposed by poor representation.
- Configuration replacing a simpler invariant.
- APIs that expose implementation details.
- Missing ownership or lifecycle boundaries.

Identify the highest-leverage structural change.

Prefer a small correction to a foundational mistake over a large
collection of local workarounds.

Do not invent architectural problems to justify a redesign.

## 5. Search for a better abstraction

Ask:

> What higher-level abstraction would make this entire problem
> disappear?

Consider whether the problem can be eliminated through:

- A better data representation.
- A stronger type.
- A simpler algorithm.
- A standard library primitive.
- A different ownership model.
- A declarative representation.
- A unified state transition.
- A more appropriate computational model.

An abstraction is useful when it removes complexity from the total
system.

Reject abstractions that merely move complexity into additional
layers.

Prefer one explicit mechanism over several interacting special cases.

Do not introduce generic frameworks for hypothetical future needs.

## 6. Design an adversarial test

Ask:

> Design the single test that breaks this design on day one.

Identify the smallest input, operation sequence, or environmental
condition most likely to expose a fundamental flaw.

Consider:

- Invalid state transitions.
- Boundary conditions.
- Repeated or reordered operations.
- Partial failures.
- Resource exhaustion.
- Concurrency and reentrancy.
- Cancellation and retry behavior.
- Integer overflow.
- Empty or degenerate inputs.
- Unexpected lifecycle changes.

Prefer testing a fundamental invariant over checking an incidental
implementation detail.

Where appropriate, use:

- Compile-time assertions.
- Runtime invariant assertions.
- Property-based testing.
- Fuzzing.
- Deterministic simulation.
- Fault injection.

Do not automatically create a large collection of unit tests.

Use `invariant-driven-testing` for detailed verification guidance.

## 7. Reduce branching and state space

Prefer designs that reduce the number of possible invalid
combinations.

Useful techniques include:

- Replacing interacting booleans with enums.
- Consolidating repeated transition logic.
- Separating independent concerns.
- Deriving values instead of duplicating state.
- Eliminating unnecessary configuration modes.
- Replacing conditional special cases with a better representation.
- Using exhaustive matching.
- Making ownership explicit.

Do not replace straightforward branching with dynamic dispatch,
lookup tables, or polymorphism solely to reduce cyclomatic complexity.

The objective is understandable behavior, not a lower metric.

## 8. Evaluate proposed changes

Before recommending a structural change, compare:

- Number of concepts required to understand the design.
- Valid and invalid states.
- Coupling between components.
- Locality of behavior.
- Runtime and memory costs.
- Ease of verification.
- Migration cost.
- New failure modes introduced.

Prefer the design with the lowest total complexity that satisfies
the requirements.

Preserve useful abstractions and boundaries.

Avoid speculative generalization.

## 9. Apply proportionally

For small, routine changes, apply these principles silently.

For complex functions or architectural changes:

1. Inspect the existing implementation.
2. Identify the actual structural problem.
3. Determine its irreducible constraints.
4. Find implicit states and transitions.
5. Consider alternative representations.
6. Identify the highest-leverage simplification.
7. Design an adversarial verification case.
8. Recommend or implement the smallest coherent change.

Do not expand an unrelated task into a repository-wide
architectural review.

## 10. Reporting

For substantial reviews, report:

### Structural findings

Identify concrete complexity sources and implicit state machines.

### Proposed representation

Describe the simpler data model, control flow, or abstraction.

### Why it is better

Explain which invalid states, branches, dependencies, or concepts
are eliminated.

### Adversarial verification

Describe the single most valuable test or invariant that could
invalidate the design.

### Migration

Identify the smallest safe sequence of changes.

Keep the report concise and grounded in actual code.

## Related skills

- `minimize-complexity`: Reduce total system complexity.
- `invariant-driven-testing`: Assert and explore correctness properties.
- `code-style`: Naming, types, formatting, and function composition.
- `performance-engineering`: Measure and optimize runtime performance.
- `context-efficient-development`: Minimize unnecessary context usage.

## Guiding principle

Find the simplest representation of the actual problem.

Make states and transitions explicit.

Eliminate unnecessary branches and invalid combinations.

Challenge the design with its strongest counterexample.
