# Markdown Basics

This document demonstrates the basic features of Markdown.

## Headings

Headings are created using `#` symbols. More `#` means smaller headings.

### This is a Level 3 Heading

#### This is a Level 4 Heading

##### This is a Level 5 Heading

###### This is a Level 6 Heading

## Text Formatting

You can make text **bold** using double asterisks or __double underscores__.

You can make text *italic* using single asterisks or _single underscores_.

You can combine them for ***bold and italic*** text.

You can also use ~~strikethrough~~ with double tildes.

## Lists

### Unordered Lists

- Item one
- Item two
  - Nested item A
  - Nested item B
- Item three

### Ordered Lists

1. First item
2. Second item
   1. Sub-item one
   2. Sub-item two
3. Third item

## Links and Images

Here is a [link to Google](https://www.google.com).

Here is a reference-style [link][example].

[example]: https://www.example.com

## Blockquotes

> This is a blockquote.
> It can span multiple lines.
>
> > Nested blockquotes are also possible.

## Code

Inline code uses `backticks`.

Code blocks use triple backticks:

```
function hello() {
    console.log("Hello, World!");
}
```

## Horizontal Rules

Three or more dashes create a horizontal rule:

---

## Tables

| Column 1 | Column 2 | Column 3 |
|----------|:--------:|---------:|
| Left     | Center   | Right    |
| Aligned  | Aligned  | Aligned  |
| Data     | Data     | Data     |

## Task Lists

- [x] Completed task
- [ ] Incomplete task
- [ ] Another task to do

## Escaping Characters

Use backslash to escape special characters: \*not italic\* and \#not a heading.

---

> End of Markdown basics demonstration.
