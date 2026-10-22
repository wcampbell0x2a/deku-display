# Sphinx

To make HTML from the reStructuredText output, use Sphinx with the sphinxcontrib-bitfield
extension.

1. Install the extension:
   ```console
   $ pip install sphinx sphinxcontrib-bitfield
   ```
2. Add the extension to `conf.py`:
   ```python
   extensions = ["sphinxcontrib.bitfield"]
   ```
3. Write the page into the Sphinx source directory, and add it to a `toctree`:
   ```console
   $ deku-display-cli src/*.rs --format rst --output docs/protocol.rst
   ```
4. Build the HTML:
   ```console
   $ sphinx-build docs docs/_build
   ```
