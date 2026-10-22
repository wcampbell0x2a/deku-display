# Development

All patches and merge requests are welcome!

```console
$ just build              # build all the crates
$ just test               # run all the tests
$ just book               # build this book, with new example pages
```

`just book-examples` makes the pages in `book/src/examples` from the files in
`docs/examples`. These pages are not in git. The book workflow makes them again each time
it builds the book.

To add an example, add a `.rs` file to `docs/examples`, and add its page to
`book/src/SUMMARY.md`.
