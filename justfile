build:
    cargo build --locked --workspace

test:
    cargo test --locked --workspace

# Make the example pages of the book from docs/examples. Needs `uv`.
book-examples:
    uv run --quiet scripts/book_examples.py docs/examples book/src/examples

# Build the book with new example pages. Needs `mdbook`.
book: book-examples
    mdbook build book
