# dioxus-gallery

Storybook for Dioxus — a domain-agnostic component previewer and isolator.

The crate is a pure `dioxus` library: it renders a registry of *stories*
(isolated component previews) in a browsable shell, or a single story in a bare
frame. It knows nothing about any specific application — no `web-sys`, no
`wasm-bindgen`, no app crates. Consumers supply their own stories and the thin
binary glue (URL routing, stylesheet, `launch`).

## Usage

Add it as a git dependency, pinned to a tag:

```toml
[dependencies]
dioxus-gallery = { git = "https://github.com/clemenscodes/dioxus-gallery", tag = "v0.1.0" }
```

Build a `StoryRegistry` from your stories and hand it to `Gallery` (browsable
shell) or `StoryFrame` (single isolated story). See the public exports in
`src/lib.rs`: `Gallery`, `StoryFrame`, `Story`, `StoryRegistry`,
`StoryComponent`, `StoryGroup`, `GalleryMode`, `GalleryView`, `FramePath`,
`ViewportPreset`.
