// Darko Template
// Dark theme template
#import "@preview/cmarker:0.1.8"
#import "@preview/mitex:0.2.6": mitex

// Get system inputs
#let filepath = sys.inputs.at("filepath", default: "input.md")
#let language = sys.inputs.at("language", default: "en")
#let show-toc = sys.inputs.at("toc", default: "false") == "true"

// Front matter inputs
#let has-frontmatter = sys.inputs.at("has_frontmatter", default: "false") == "true"
#let fm-title = sys.inputs.at("fm_title", default: none)
#let fm-subtitle = sys.inputs.at("fm_subtitle", default: none)
#let fm-author = sys.inputs.at("fm_author", default: none)
#let fm-date = sys.inputs.at("fm_date", default: none)
#let fm-tags = sys.inputs.at("fm_tags", default: none)
#let fm-version = sys.inputs.at("fm_version", default: none)
#let fm-logo = sys.inputs.at("logo", default: none)
#let fm-participants = sys.inputs.at("participants", default: none)

// Dieter Rams color palette for dark theme
#let rams-white = rgb("f7f8f6ff")      // Text color (whitish)
#let rams-light-grey = rgb("d9d2c6ff")  // Light accents
#let rams-dark-grey = rgb("4a4a4aff")   // Medium elements
#let rams-black = rgb("1f1f1fff")       // Background (dark)
#let rams-green = rgb("736b1eff")       // Accent color
#let rams-brown = rgb("8b7355ff")       // Secondary accent
#let rams-red = rgb("ed3f1cff")         // Highlight color
#let rams-orange = rgb("ed8008ff")      // Warning/emphasis color

// Parse tags from comma-separated string
#let tags-list = if fm-tags != none { fm-tags.split(",") } else { () }
#let participants-list = if fm-participants != none { fm-participants.split(",") } else { () }

// Extract filename from filepath (remove path and .md extension)
#let filename = {
  let path-parts = filepath.split("/")
  let file = path-parts.last()
  if file.ends-with(".md") {
    file.slice(0, file.len() - 3)
  } else if file.ends-with(".temp.md") {
    file.slice(0, file.len() - 8)
  } else {
    file
  }
}

// Use front matter data or defaults
#let document-author = if fm-author != none { fm-author } else { none }
#let document-title = if fm-title != none { fm-title } else { filename }
#let document-subtitle = if fm-subtitle != none { fm-subtitle } else { none }

// Parse date
#let document-date = if fm-date != none {
  let date-str = fm-date
  if date-str.len() == 10 and date-str.contains("-") {
    let parts = date-str.split("-")
    if parts.len() == 3 {
      datetime(year: int(parts.at(0)), month: int(parts.at(1)), day: int(parts.at(2)))
    } else {
      datetime.today()
    }
  } else {
    datetime.today()
  }
} else {
  datetime.today()
}

// Set document properties
#set document(
  author: if document-author != none {document-author} else {""},
  title: document-title,
  keywords: if fm-tags != none { (if document-author != none {document-author} else {""}, if document-title != none {document-title} else {""}, "md-pdf", ..tags-list) } else { (if document-author != none {document-author} else {""}, if document-title != none {document-title} else {""}, "md-pdf") },
  date: document-date
)

// Basic properties - dark background
#set page(
  margin: (top: 3cm, bottom: 3cm, left: 3cm, right: 2.5cm),
  fill: rams-black
)

// Header and footer
#set page(
  header: context(if here().page() >= 2 [
    #set text(9pt, fill: rams-light-grey)
    #table(
      columns: (80%, 20%),
      stroke: none,
      inset: -0.5em,
      align: (x, y) => (left+bottom, right+top).at(x),
      [#smallcaps[#document-title] #if document-subtitle != none {[| #smallcaps[#document-subtitle] ]}],
      [#if fm-logo != none {[#v(1.2cm)#image(fm-logo,width:2cm)]}]
    )
    #if fm-logo != none {[
      #line(start: (-0.5em, 0cm), length: 85%, stroke: (paint: rams-dark-grey, thickness: 0.5pt))
    ]} else {[
      #line(start: (-0.5em, 0cm), length: 101%, stroke: (paint: rams-dark-grey, thickness: 0.5pt))
    ]}
  ]),
  footer: context(if here().page() >= 2 [
    #line(length: 100%, stroke: (paint: rams-dark-grey, thickness: 0.5pt))
    #set text(9pt, fill: rams-light-grey)
    #v(0.2em)
    #if document-author != none {[#document-author #h(1fr)]} #document-date.display() #h(1fr) #context counter(page).display("1 / 1", both: true)
  ])
)

// Font & language - white text on dark background
#set text(
  font: ("Libertinus Serif", "Liberation Serif"),
  fallback: true,
  lang: language,
  size: 11pt,
  fill: rams-white
)

// Heading styling
#show heading: set block(above: 1.2em, below: 1.2em)
#set heading(numbering: "1.1")

#show heading.where(level: 1): it => {
  set text(size: 18pt, weight: "bold", fill: rams-white)
  set block(above: 1.2em, below: 1.2em)
  if it.numbering != none {
    let num = numbering(it.numbering, ..counter(heading).at(it.location()))
    block(
      fill: rams-green.darken(20%),
      inset: (x: 12pt, y: 8pt),
      radius: 4pt,
      width: 100%,
      stroke: 1pt + rams-green
    )[
      #text(weight: "bold", fill: rams-white)[#num] #h(8pt) #it.body
    ]
  } else {
    block(
      fill: rams-green.darken(20%),
      inset: (x: 12pt, y: 8pt),
      radius: 4pt,
      width: 100%,
      stroke: 1pt + rams-green
    )[
      #it.body
    ]
  }
}

#show heading.where(level: 2): it => {
  set text(size: 15pt, weight: "bold", fill: rams-orange)
  set block(above: 1.3em, below: 0.9em)
  if it.numbering != none {
    let num = numbering(it.numbering, ..counter(heading).at(it.location()))
    [#text(weight: "bold", fill: rams-orange)[#num] #h(6pt) #it.body]
  } else {
    [#it.body]
  }
  v(-7pt)
  line(length: 70%, stroke: 2pt + rams-orange.lighten(20%))
}

#show heading.where(level: 3): it => {
  set text(size: 13pt, weight: "semibold", fill: rams-brown)
  if it.numbering != none {
    let num = numbering(it.numbering, ..counter(heading).at(it.location()))
    [#text(weight: "semibold", fill: rams-brown)[#num] #h(4pt) #it.body]
  } else {
    [#it.body]
  }
  v(-6pt)
  line(length: 50%, stroke: 1pt + rams-brown.lighten(30%))
}

// Link color
#show link: it => text(fill: rams-green.lighten(20%), weight: "medium", it)

// Code blocks
#show raw: set text(
  font: ("Fira Code", "DejaVu Sans Mono"),
  fallback: true
)
#show raw.where(block: false): it => {
  box(
    fill: rams-dark-grey.darken(40%),
    inset: (x: 4pt, y: 2pt),
    radius: 3pt,
    stroke: 0.5pt + rams-dark-grey.darken(10%)
  )[
    #text(fill: rams-light-grey, weight: "medium", size: 9pt)[#it]
  ]
}
#show raw.where(block: true): it => {
  set text(size: 9pt, fill: rams-light-grey)
  block(
    fill: rams-dark-grey.darken(40%),
    width: 100%,
    inset: 12pt,
    radius: 5pt,
    stroke: 1pt + rams-dark-grey.darken(10%)
  )[
    #it
  ]
}

// Lists
#set list(indent: 1em, marker: ([•], [◦], [▪]))
#set enum(indent: 1em)
#show list: it => {
  set text(fill: rams-white)
  it
}
#show enum: it => {
  set text(fill: rams-white)
  it
}

// Emphasis
#show emph: set text(style: "italic", fill: rams-light-grey, weight: "medium")
#show strong: set text(weight: "bold", fill: rams-green.lighten(10%))

// Quotes
#show quote: it => {
  set text(fill: rams-light-grey, style: "italic")
  block(
    fill: rams-dark-grey.lighten(5%),
    inset: (left: 12pt, rest: 10pt),
    radius: 4pt,
    stroke: (left: 3pt + rams-orange, rest: 1pt + rams-dark-grey.lighten(20%))
  )[#it]
}

// Table styling
#show table: it => {
  set text(size: 10pt, fill: rams-white)
  set table(
    stroke: 1pt + rams-dark-grey.lighten(30%),
    fill: rams-dark-grey.lighten(10%)
  )
  it
}

// Figure captions
#set figure(numbering: "1", supplement: [Figure])
#set figure.caption(separator: " — ")
#show figure.caption: it => {
  set text(size: 9pt, style: "italic", fill: rams-light-grey)
  block(
    fill: rams-dark-grey.lighten(10%),
    inset: 8pt,
    width: 100%,
    radius: 3pt,
    stroke: 1pt + rams-dark-grey.lighten(30%)
  )[
    #it
  ]
}

// Badge function with dark theme colors
#let badge(content, index: 0) = {
  let colors = (rams-green, rams-brown, rams-orange, rams-red)
  let color = colors.at(calc.rem(index, colors.len()))
  box(
    inset: (x: 6pt, y: 3pt),
    radius: 3pt,
    fill: color.darken(30%),
    stroke: (paint: color, thickness: 0.5pt)
  )[
    #text(weight: "medium", size: 8pt, fill: rams-white)[#content]
  ]
}

// Show basic document metadata if front matter exists
#if has-frontmatter [
  #v(1em)
  // title
  #if fm-title != none [
    #align(center)[
      #text(size: 22pt, weight: "bold", fill: rams-white)[#fm-title]
    ]
    #v(0.5em)
  ]
  // subtitle
  #if fm-subtitle != none [
    #align(center)[
      #text(size: 15pt, style: "italic", fill: rams-light-grey)[#fm-subtitle]
    ]
    #v(0.8em)
  ]
  // logo
  #if fm-logo != none {[#figure(image(fm-logo,width:4cm))]}
  // metadata (author, date, version)
  #let metadata = ()
  #if document-author != none { metadata.push(document-author) }
  #if document-date != none { metadata.push(document-date.display()) }
  #if fm-version != none { metadata.push(fm-version) }

  #if metadata.len() > 0 [
    #set text(11pt, fill: rams-light-grey)
    #align(center)[
      #for (i, data) in metadata.enumerate() [
        #data
        #if i < metadata.len() - 1 [ • ]
      ]
    ]
    #v(0.8em)
  ]

  #if fm-tags != none and tags-list.len() > 0 or fm-participants != none and participants-list.len() > 0 [
    #align(center)[#line(length: 90%, stroke: 1pt + rams-light-grey)]
  ]
  // tags
  #if fm-tags != none and tags-list.len() > 0 [
    #table(
      columns: (20%, 80%),
      align: (right+horizon, left+horizon),
      stroke: none,
      [#smallcaps[#text(fill: rams-light-grey)[
        #if language == "de" {[
          Tags
        ]} else if language == "fr" {[
          Balises
        ]} else {[
          Tags
        ]}
      ]]],
      [
        #for (i, tag) in tags-list.enumerate() [
          #badge(tag.trim(), index: i)
        ]
      ]
    )
  ]
  // participants
  #if fm-participants != none and participants-list.len() > 0 [
    #table(
      columns: (20%, 80%),
      align: (right+horizon, left+horizon),
      stroke: none,
      [#smallcaps[#text(fill: rams-light-grey)[
        #if language == "de" {[
          Teilnehmer
        ]} else if language == "fr" {[
          Participants
        ]} else {[
          Participants
        ]}
      ]]],
      [
        #for (i, participants) in participants-list.enumerate() [
          #badge(participants.trim(), index: participants-list.len() - i - 1)
        ]
      ]
    )
  ]

  // separator
  #align(center)[#line(length: 90%, stroke: 1pt + rams-light-grey)]
]

// Table of contents
#if show-toc [
  #set text(fill: rams-light-grey, size: 16pt, weight: "bold")
  [Contents]
  #v(0.5em)
  #line(length: 100%, stroke: 1pt + rams-green)
  #v(0.5em)
  #set text(weight: "medium", fill: rams-white, size: 10pt)
  #show outline.entry: it => {
    set text(fill: rams-light-grey)
    it
  }
  #outline(indent: auto)
  #pagebreak()
]

#cmarker.render(
  read(filepath),
  scope: (image: (path, alt: none) => image(path, alt: alt)),
  math: mitex
)
