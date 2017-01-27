# Search Spell

**Toy Learning Project**

This is a simple gem that makes use of [Ruru](https://github.com/d-unseductable/ruru) to implement a simple scraper in Rust that fetches spelling suggestions from search engines.

The Rust code can make requests in parallel and return them to the Ruby code.

## Todo

- [ ] Fix the `itself_name` unused warning.
- [ ] Use [`call_without_gvl`](https://d-unseductable.github.io/ruru/ruru/struct.Thread.html#method.call_without_gvl) to see if it's any faster. (was getting a seg fault).
- [ ] Add Bing.
- [ ] Compare with FFI implementation.
