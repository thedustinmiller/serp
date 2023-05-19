## Purpose
The rust `format!` macro requires the string template to be passed as a string literal at compile time.
This crate uses the well known template language handlebars to allow for everything to be passed at runtime. 
Using the entirety of handlebars is definitely overkill for simply making an ergonomic string interpolation function,
but I'm far more confident in the correctness of a well tested and popular library than I would be if I rolled my own.
I make no modifications to the syntax of handlebars, so unfortunately I can't replicate the syntax of `format!` exactly.

## Usage

You can initialize an empty template with `Template::new()` and then modify the template and/or push new key/value pairs.
This isn't much more succinct than handlebars directly, but it's here for completeness.
```rust
let mut s = Serp::default();
s.template = "{{sample}} {{string}}".to_string();
s.push("sample".into(), "hello".into());
s.push("string".into(), "world".into());

assert_eq!(s.format(), "hello world");
```

The primary function is `serp` which takes a template as a `String` and a `HashMap<String, String>` of key/value pairs.
```rust
let template = "Hello, {{name}}".to_string();
let map = HashMap::from([("name".into(), "world".into())]);

assert_eq!(serp(&template, &map), "Hello, world".to_string());
```

The final methods are the final product, shorthand ways to use `serp`. `t` takes a template as a `&str` 
and an array of key/value tuples as a `&[(&str, &str)]` 
```rust
let t = t("{{sample}} {{string}}", &[("sample", "Hello"), ("string", "World")]);
assert_eq!(t, "Hello World");
```

Finally, the laziest example, which uses a little convention to be both standard handlebars and succinct. `a` simply 
takes a template as a `&str` and an array of strings in `&[&str]`. The template brackets are numbered, 0 indexed.
```rust
let a = a("{{0}} {{1}}", &["Hello", "World"]);
assert_eq!(a, "Hello World");
```

## License
Dual licensed under MIT and Apache 2.0