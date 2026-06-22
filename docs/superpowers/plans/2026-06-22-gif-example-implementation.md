# GIF Example Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enable the image-display example to render static PNG files and animated GIF files on a HUB75 matrix.

**Architecture:** Keep all image format handling in `examples/show_image.rs`. Decode still images once; decode GIF frames with their frame delay and repeatedly present their RGB8 pixels until a duration expires or SIGINT requests shutdown.

**Tech Stack:** Rust 2024, `image` 0.25 GIF decoder, `clap`, `ctrlc`, `rust-hub75-matrix`.

## Global Constraints

- The library crate has no image-decoding dependency.
- GIF support is enabled only in the `image` dev-dependency.
- Frame dimensions must match the logical matrix unless `--resize` is supplied.
- SIGINT exits normally, clears the matrix, and waits 100ms.

---

### Task 1: Enable GIF decoding and replace the PNG-only example

**Files:**
- Modify: `Cargo.toml`
- Create: `examples/show_image.rs`
- Delete: `examples/show_png.rs`
- Modify: `README.md`

**Interfaces:**
- Consumes: `Matrix::present_rgb(&mut self, &[u8]) -> Result<()>` and `Matrix::clear(&mut self) -> Result<()>`.
- Produces: `show_image <path> [--resize] [--duration seconds]`.

- [ ] **Step 1: Write the failing build expectation**

Run: `cargo check --example show_image`

Expected: failure because the example and GIF feature do not exist.

- [ ] **Step 2: Enable GIF support**

```toml
image = { version = "0.25", default-features = false, features = ["gif", "png"] }
```

- [ ] **Step 3: Implement image playback**

Use `ImageReader::open(path).with_guessed_format()` for static images and `GifDecoder::into_frames()` for GIF frames. Convert each frame to RGB8, resize only with `--resize`, and use `frame.delay().numer_denom_ms()` to sleep until the next presentation.

- [ ] **Step 4: Update documentation**

Replace `show_png` commands and description with `show_image`; document PNG and animated GIF input.

- [ ] **Step 5: Verify compilation and format**

Run: `cargo fmt --check && cargo check --examples`

Expected: formatting and compile success on a Linux Raspberry Pi build environment.

### Task 2: Commit and publish

**Files:**
- Modify: tracked files from Task 1 and this plan.

- [ ] **Step 1: Review the diff**

Run: `git diff --check && git status --short`

Expected: only GIF support, its documentation, and planning files are present.

- [ ] **Step 2: Commit**

Run: `git add Cargo.toml README.md examples/show_image.rs examples/show_png.rs docs/superpowers && git commit -m "feat: add GIF image example"`

- [ ] **Step 3: Push main**

Run: `git push origin main`

Expected: remote `main` advances with the GIF support commit.
