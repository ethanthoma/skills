---
name: code-style
description: >
  Enforce consistent code style, naming, formatting, type design,
  and function composition. Follow Google's language-specific style
  guides with the user's explicit conventions taking precedence.
  Use the project's formatter or the user's Neovim Conform setup.
  Apply when writing, editing, refactoring, or reviewing code.
---

# Code Style

Write explicit, readable, type-safe code with descriptive names,
small cohesive functions, and predictable control flow.

Draw from Google's language-specific style guides, Jonathan Blow's
emphasis on direct programming, tinygrad's compact implementation
style, and TigerBeetle's engineering discipline.

Prefer clarity, correctness, performance, and maintainability over
brevity, cleverness, or architectural ceremony.

## 1. Precedence

When conventions conflict, follow this order:

1. Explicit instructions for the current task.
2. Repository-specific AGENTS.md and established requirements.
3. The conventions in this skill.
4. Applicable Google language-specific style guides.
5. Established language and ecosystem conventions.

Preserve externally mandated names, public API contracts, serialized
field names, and framework interfaces when changing them would break
compatibility.

Do not use Google style guides to override explicit preferences
in this skill.

## 2. Naming

Names should explain what something represents or does without
requiring the reader to inspect its implementation.

- Prefer `snake_case` for identifiers where the language permits it.
- Avoid abbreviations except conventional loop indices (`i`, `j`),
  mathematical notation, and established domain terminology.
- Prefer long, descriptive names over short, ambiguous names.
- Use consistent terminology across related operations.
- Prefer nouns for values and verbs for operations.
- Name predicates to communicate their truth condition.
- Do not introduce synonyms merely for variety.
- Use proper capitalization for established acronyms in type names,
  such as `VSRState`, where language conventions permit it.

Examples:

Bad:
```rust
fn proc_buf(buf: &[u8], cfg: &Cfg) -> Vec<u8>
```

Better:
```rust
fn process_vertex_buffer(
    vertex_buffer: &[u8],
    configuration: &VertexProcessingConfiguration,
) -> Vec<u8>
```

Prefer:
- `source` / `target` over `src` / `dest`.
- `configuration` over `cfg`.
- `vertex_buffer` over `vbuf`.
- `frame_duration_seconds` over `dt`.
- `buffer_size_bytes` over `buf_sz`.
- `maximum_frame_latency_ms` over `max_lat`.

Do not expand universally recognized mathematical or technical
notation when doing so would make an algorithm harder to understand.

### Units and qualifiers

Put units and qualifiers at the end of names.

Examples:
- `timeout_seconds`
- `buffer_size_bytes`
- `latency_ms_max`
- `frame_count`
- `vertex_index`
- `texture_width_pixels`

Distinguish:
- `index`: zero-based position.
- `count`: number of elements.
- `size`: magnitude with an explicit unit when relevant.

Make conversions between indices, counts, and sizes explicit.

For division operations where rounding matters, make the intent
visible through names such as `divfloor`, `divceil`, and `divexact`.

### Related helpers

When useful, prefix a helper with the operation it supports.

Example:
```rust
fn read_sector(...)
fn read_sector_callback(...)
```

Do not force this convention when it produces misleading names.

## 3. Types over comments

Prefer encoding meaning and constraints in types rather than
explaining primitive values through comments.

Use:
- Explicit parameter and return types.
- Strong types for semantically different quantities.
- Enums for distinct states.
- Exhaustive matching.
- Constructors that establish invariants.
- Sized integer types where width matters.
- Ownership and lifetime constraints where applicable.

Prefer:
```rust
struct TextureDimensions {
    width_pixels: u32,
    height_pixels: u32,
}

fn allocate_texture(
    dimensions: TextureDimensions,
) -> Texture
```

Over:
```rust
// Width and height are pixel counts.
fn allocate_texture(width: u32, height: u32) -> Texture
```

Do not create a wrapper type for every primitive. Introduce
types when they eliminate ambiguity, invalid states, or repeated
validation.

Prefer options structs or named arguments when several parameters
have the same type and their order could be confused.

## 4. Small, single-purpose functions

Each function should perform one coherent operation.

Prefer:
- Small functions with descriptive names.
- Explicit inputs and outputs.
- Local reasoning.
- Limited side effects.
- Clear ownership.
- Straightforward control flow.
- Pure functions when practical.

Compose these operations into more complex behavior.

Example:

```rust
fn calculate_surface_normal(...) -> Vec3 {
    // ...
}

fn calculate_fresnel_reflectance(...) -> f32 {
    // ...
}

fn calculate_surface_lighting(...) -> Vec3 {
    // ...
}

fn shade_surface(...) -> Vec3 {
    let surface_normal = calculate_surface_normal(...);
    let fresnel_reflectance =
        calculate_fresnel_reflectance(...);
    let surface_lighting =
        calculate_surface_lighting(...);

    // Combine the results.
}
```

A function should be extracted because it represents a meaningful
operation, not merely because another function exceeds an arbitrary
line count.

Avoid:
- Trivial pass-through wrappers.
- Helpers that obscure a single obvious expression.
- Excessive indirection.
- Deep call chains without meaningful boundaries.
- Splitting cohesive behavior across many files.
- Generic abstractions with only hypothetical uses.

A long function with one coherent responsibility can be preferable
to several poorly named fragments.

## 5. Function composition

Prefer composing independent operations over building large
objects with many responsibilities.

Favor:
- Data flowing through explicit transformations.
- Independent operations with narrow interfaces.
- Functions that can be reasoned about in isolation.
- Separation of computation from side effects where useful.
- Composition at the level where the complete behavior is visible.

Avoid:
- Excessive dependency injection.
- Factory and manager hierarchies without a concrete need.
- Inheritance-based behavior composition.
- Frameworks built around trivial operations.
- Callbacks when direct control flow is clearer.

Do not sacrifice performance to enforce a functional programming
style. Mutation, loops, and in-place operations are appropriate
when they improve clarity or efficiency.

## 6. Direct programming

Follow the useful aspects of tinygrad and Jonathan Blow's
programming philosophy:

- Prefer straightforward implementations.
- Keep important data transformations visible.
- Avoid unnecessary layers.
- Minimize hidden control flow.
- Avoid speculative generalization.
- Use the language's basic constructs effectively.
- Keep related operations close together.
- Introduce abstractions only when they reduce total complexity.

Avoid clever one-liners that require more effort to understand
than their expanded equivalents.

Prefer explicit intermediate values when they clarify computation.

Do not optimize for the fewest lines of code.

For broader architectural decisions, use `minimize-complexity`.

## 7. Safety and invariants

Follow Tiger-style engineering principles:

- Make illegal states unrepresentable where practical.
- Assert internal invariants.
- Handle external failures explicitly.
- Bound loops, queues, allocations, and resource consumption.
- Keep variables in the smallest useful scope.
- Avoid duplicated mutable state.
- Keep resource allocation near cleanup.
- Prefer explicit control flow.
- Account for integer overflow and numerical boundaries.

Do not use assertions to validate untrusted external input.

Prefer compile-time guarantees when they provide equivalent or
stronger protection than runtime checks.

Use `invariant-driven-testing` for detailed verification guidance.

## 8. Comments and documentation

Prefer descriptive names, types, and function boundaries over
comments explaining what the code does.

Comments should explain information that cannot be expressed
clearly through code.

Appropriate comments include:
- Non-obvious hardware or platform constraints.
- Mathematical derivations.
- Safety requirements.
- External protocol requirements.
- Important performance assumptions.
- Reasons for surprising implementation choices.

Avoid:
- Narrating obvious statements.
- Repeating function names.
- Section comments used to compensate for poor organization.
- Describing historical implementation changes in ordinary code.
- Large documentation blocks that duplicate the type signature.

Use actionable TODO comments:

```text
TODO: Handle device loss before recreating the render pipeline.
```

Do not remove comments that document essential invariants or
external constraints.

## 9. Formatting

The user's Neovim Conform configuration is the preferred source
of formatter selection.

Before formatting:

1. Inspect repository formatter configuration.
2. Inspect the user's Neovim Conform mappings when available.
3. Identify the formatter configured for the language.
4. Run that formatter on changed files.
5. Verify the formatter did not introduce unrelated changes.

The user's Neovim configuration is maintained separately from
the NixOS Home Manager configuration.

Check the actual configuration instead of assuming which
formatter is installed.

Relevant locations may include:
- `~/.config/nvim/`
- Repository formatter configuration.
- `modules/home-manager/neovim.nix` in the NixOS configuration.

Prefer the configured formatter's CLI when available.

For example, use `rustfmt` when Conform configures rustfmt for
Rust. Do not assume that this mapping exists without checking.

Do not hand-format code against the formatter's output.

Do not introduce a new formatter merely because it is associated
with Google's style guides.

On NixOS, do not assume tools are available on PATH. Use the
repository's development environment or an appropriate Nix shell
when necessary.

If the configured formatter cannot be run, report the limitation.

## 10. Language-specific conventions

Follow the applicable Google style guide where one exists,
subject to the precedence rules above.

For languages without an applicable Google guide, use established
language conventions and the repository's configuration.

### Rust

- Prefer idiomatic Rust ownership and borrowing.
- Use descriptive `snake_case` function and variable names.
- Use PascalCase for types and variants as required by Rust style.
- Prefer enums over loosely related boolean state.
- Prefer explicit error handling.
- Avoid unnecessary allocations and cloning.
- Keep unsafe code minimal and document safety invariants.
- Preserve exhaustive matching.

### Nix

- Follow the configured Nix formatter.
- Preserve Nix language syntax and established option names.
- Prefer small, explicit modules.
- Avoid unnecessary indirection and repeated configuration.
- Use descriptive bindings.

### Python

- Follow Google's Python style guide where compatible.
- Use descriptive `snake_case` identifiers.
- Prefer explicit type annotations.
- Keep data transformations understandable.
- Avoid unnecessary classes and decorators.
- Follow the configured formatter.

### WGSL and GPU code

- Prefer descriptive shader identifiers.
- Make coordinate spaces and units explicit.
- Keep buffer layouts and alignment requirements visible.
- Avoid unnecessary shader abstractions.
- Consider register pressure, divergence, and memory access.
- Preserve performance-sensitive operations when justified.

Use `webgpu-rendering` for GPU-specific correctness guidance.

### Other languages

Follow the repository's established language requirements,
formatter, and applicable Google style guide.

Do not introduce a new language solely for stylistic reasons.

## 11. File organization

- Put important concepts near the top of a file.
- Put `main()` first when appropriate.
- Keep related functions and types together.
- Organize struct fields logically.
- Use alphabetical ordering when no logical ordering exists.
- Keep allocation and cleanup responsibilities close.
- Avoid splitting files solely to reduce their length.
- Prefer files organized around cohesive functionality.

## 12. Scope

Apply these conventions to new and modified code.

Do not reformat or rename an entire repository for an unrelated task.

When modifying an existing function, improve names and structure
where it directly helps the change.

Avoid introducing mixed conventions within the same component.

When a requested change requires renaming an identifier, update
all affected callers within the permitted scope.

Do not silently break public APIs or external contracts.

## 13. Verification

Before completing a coding task:

1. Check naming and type clarity.
2. Check function responsibilities and composition.
3. Confirm comments add information.
4. Run the configured formatter.
5. Run relevant static checks and verification.
6. Review the diff for unrelated formatting or refactoring.

Apply these rules without producing a lengthy style report.

Mention deviations only when they are significant or intentional.

## Related skills

- `minimize-complexity`: Architectural simplicity and abstraction costs.
- `invariant-driven-testing`: Assertions and verification.
- `performance-engineering`: Measurement-based optimization.
- `context-efficient-development`: Efficient context and tool usage.
- `webgpu-rendering`: WebGPU and WGSL development.

## Guiding principle

Write code whose intent is visible through its names, types,
data flow, and function boundaries.

Prefer explicit, composable operations over hidden complexity.

Let the formatter handle formatting.
