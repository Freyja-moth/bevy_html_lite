# HTML LITE
Like html, but only for text.

Sections can be created by using the sections macro.

```rust
// Note the curly brackets
let sections = sections!(<i> { "Hello there" } </i>);
```

Which will expand to this.
```rust
let sections = Sections::from_iter([
    Sections::new("Hello there", false, true, None);
])
```

> The macro itself isn't required, but it is strongly recommended as it makes life a lot easier.

You can even do formatting inside the strings!
```rust
let x = "TADA!";
let sections = sections!({ "{x}" });
```

And also there's color!
```rust
let sections = sections!(<span color = "#5BCEFA"> { "We" } </span> <span color = "#F5A9B8"> { "now" } </span> { "have" } <span color = "#F5A9B8"> { "Color" } </span> <span color = "#5BCEFA"> { "!" } </span>)
```

> The default color should be set with [DefaultTextColor](https://github.com/Freyja-moth/bevy_html_lite/blob/main/src/plugin.rs#L24) when using DefaultHtmlLiteDisplayPlugin.

## Plugins
There is a default plugin called [DefaultHtmlLiteDisplayPlugin](https://github.com/Freyja-moth/bevy_html_lite/blob/main/src/plugin.rs#L34) that uses observer to spawn and despawn text. It largely exists to give users an example on how to use this crate and I would not recommended using it beyond playing around. You should probably create your own implementation for any serious project as what you create will be far more likely to suite you tastes.

### Implemented stuff 
Currently the only implemented tags are `<i>` and `<b>`. You can still use other tags (which is useful for colors), they simply won't have any special effects (for now).

### Things yet to do
For the time being you can't load parse from a file due to the way the macro works.
More tags for fancy stuff!

#### For those reading the source code
Yes I am aware this is not the best code in the world, this was written in about a day. If you know a better way of doing it please let me know with a pull request or issue.
