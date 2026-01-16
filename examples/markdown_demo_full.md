---
title: Markdown Demo
version: 1.0.0
author: ratatui-toolkit
description: A comprehensive demonstration of markdown rendering features
---

# Markdown Renderer Showcase

This document demonstrates all features supported by the **MarkdownRenderer**.

## 1. Frontmatter

YAML frontmatter at the top is parsed and displayed as a collapsible section. This is commonly used in static site generators like Jekyll, Hugo, and Docusaurus.

---

## 2. Text Formatting

**Bold text** using double asterisks

*Italic text* using single asterisks

***Bold and italic*** combined

`Inline code` with monospaced styling

~~Strikethrough~~ (not currently rendered but parsed)

---

## 3. Links

[ratatui-toolkit GitHub](https://github.com/anomalyco/ratatui-toolkit)

[Inline link with **bold** text](https://example.com)

<https://autolink-url.com>

---

## 4. Code Blocks

### Rust

```rust
fn main() {
    let greeting = "Hello, World!";
    println!("{}", greeting);
}
```

### Python

```python
def greet(name: str) -> str:
    """Return a greeting message."""
    return f"Hello, {name}!"

# Example usage
print(greet("ratatui"))
```

### JavaScript

```javascript
const greeting = "Hello, World!";
console.log(greeting);

function add(a, b) {
    return a + b;
}
```

### Go

```go
package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}
```

### Shell

```bash
#!/bin/bash
echo "Hello, World!"
cargo build --release
```

---

## 5. Tables

### Simple Table

| Feature | Status | Notes |
|---------|--------|-------|
| Bold | ✅ | `**text**` |
| Italic | ✅ | `*text*` |
| Code | ✅ | `` `code` `` |
| Tables | ✅ | `\|col\|` syntax |

### Data Table

| Language | Year | Paradigm |
|----------|------|----------|
| Rust | 2010 | Multi-paradigm |
| Python | 1991 | Multi-paradigm |
| Go | 2009 | Imperative |
| JavaScript | 1995 | Multi-paradigm |

### Right-aligned Table

| Name | Score | Grade |
|-----:|------:|:-----:|
| Alice | 95 | A |
| Bob | 87 | B+ |
| Charlie | 92 | A- |

---

## 6. Lists

### Unordered Lists

- First item
- Second item
  - Nested level 1
    - Nested level 2
      - Deeply nested
  - Back to level 1
- Third item
- Fourth item

### Ordered Lists

1. First ordered item
2. Second ordered item
   1. Nested ordered
   2. Another nested
3. Third item
4. Fourth item

### Mixed List

1. First numbered
2. Second numbered
   - Bullet point
   - Another bullet
     1. Numbered inside
     2. Another
   - Back to bullets
3. Third numbered

---

## 7. Blockquotes

### Simple Quote

> The only way to do great work is to love what you do.
> — Steve Jobs

### Nested Blockquotes

> This is a top-level quote.
>
> > This is a nested quote.
> >
> > > And another level deeper!
>
> Back to the first level.

### Code in Quote

> Here's a code example:
>
> ```rust
> fn main() {
>     println!("Quote with code!");
> }
> ```

### Multi-line Quote

> Markdown is a lightweight markup language with plain text formatting syntax.
> It is designed so that it can be converted to HTML and many other formats
> using a tool by the same name.

---

## 8. Headings

### Heading Level 1

Usually used for page titles.

### Heading Level 2

Used for major sections.

#### Heading Level 3

For subsections.

##### Heading Level 4

Even smaller subsections.

###### Heading Level 5

Rarely used.

###### Heading Level 6

The smallest heading.

---

## 9. Horizontal Rules

---

***

---

## 10. Mixed Content Example

### Complete Document Structure

# Main Title

## Introduction

This is a **comprehensive** demo showing *all* markdown features supported by our renderer.

### Features Overview

| Category | Features |
|----------|----------|
| Text | Bold, Italic, Inline Code |
| Blocks | Headings, Quotes, Lists |
| Code | Fenced blocks with language |
| Tables | With alignment support |
| Links | Standard and autolinks |

### Usage Example

Here's how you might use the markdown renderer:

```rust
use ratatui_toolkit::render_markdown;

let markdown = "# Hello\n\n**Bold** and *italic*.";
let text = render_markdown(markdown, Some(80));
```

### Notes

> **Warning**: Always test your markdown with the actual renderer.
>
> Different parsers may have slight variations in behavior.

---

## 11. Deeply Nested Structures

1. Level 1
   - Level 2
     1. Level 3 ordered
     2. Level 3 ordered
   - Level 2 bullet
     > Level 2 quote
     > > Level 3 quote
   - Level 2 bullet
2. Level 1 continued
3. Level 1 end

---

*End of Markdown Demo*
