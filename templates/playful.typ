// Playful Template
// Colorful inspired by Dieter Rams
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

// Dieter Rams inspired color palette
#let rams-white = rgb("ffffffff")
#let rams-light-grey = rgb("d9d2c6ff")
#let rams-dark-grey = rgb("4a4a4aff")
#let rams-black = rgb("1f1f1fff")
#let rams-green = rgb("736b1eff")
#let rams-brown = rgb("8b7355ff")
#let rams-red = rgb("ed3f1cff")
#let rams-orange = rgb("ed8008ff")

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

// basic properties
#set page(
  margin: (top: 3.5cm, bottom: 3cm, left: 3cm, right: 3cm),
  fill: rams-white
)

// header and footer
#set page(
  header: context(if here().page() >= 2 [
    #set text(9pt, fill: rams-dark-grey)
    #table(
      columns: (80%, 20%),
      stroke: none,
      inset: -0.5em,
      align: (x, y) => (left+bottom, right+top).at(x),
      [#smallcaps[#document-title] #if document-subtitle != none {[| #smallcaps[#document-subtitle] ]}],
      [#if fm-logo != none {[#v(1.2cm)#image(fm-logo,width:2cm)]}]
    )
    #if fm-logo != none {[
      #line(length: 85%, stroke: (paint: rams-light-grey, thickness: 0.5pt))
    ]} else {[
      #line(length: 101%, stroke: (paint: rams-light-grey, thickness: 0.5pt))
    ]}
  ]),
  footer: context(if here().page() >= 2 [
    #line(length: 100%, stroke: (paint: rams-light-grey, thickness: 0.5pt))
    #set text(8pt, fill: rams-dark-grey)
    #v(0.2em)
    #if document-author != none {[#document-author #h(1fr)]} #document-date.display() #h(1fr) #context counter(page).display("1 / 1", both: true)
  ])
)

// font & language
#set text(
  font: ("Fira Sans", "Liberation Sans"),
  fallback: true,
  lang: language,
  size: 10pt,
  fill: rams-black
)

// heading
#show heading: set block(above: 1.4em, below: 0.8em)
#set heading(numbering: "1.1")

#show heading.where(level: 1): set text(
  size: 16pt,
  weight: "medium",
  fill: rams-dark-grey
)
#show heading.where(level: 2): set text(
  size: 14pt,
  weight: "medium",
  fill: rams-brown
)
#show heading.where(level: 3): set text(
  size: 12pt,
  weight: "medium",
  fill: rams-orange
)
#show heading.where(level: 4): set text(
  size: 11pt,
  weight: "semibold",
  fill: rams-red
)

// link color
#show link: it => text(fill: rams-green, it)

// code blocks
#show raw: set text(
  font: ("Fira Code", "DejaVu Sans Mono"),
  fallback: true,
  size: 8pt
)
#show raw.where(block: false): set text(weight: "medium", fill: rams-orange.darken(20%), size: 8pt)
#show raw.where(block: true): block.with(
  fill: rams-brown.lighten(90%),
  inset: 12pt,
  radius: 3pt,
  width: 100%,
  stroke: (paint: rams-brown.lighten(60%), thickness: 0.5pt)
)

// Lists
#set list(indent: 1em, marker: ([•], [◦], [▪]))
#set enum(indent: 1em)

// Emphasis
#show emph: set text(style: "italic", fill: rams-orange.darken(10%))
#show strong: set text(weight: "semibold", fill: rams-red)

// badge function
#let badge(content, index: 0) = {
  let colors = (rams-green, rams-brown, rams-orange, rams-red)
  let color = colors.at(calc.rem(index, colors.len()))
  box(
    inset: (x: 6pt, y: 3pt),
    radius: 2pt,
    fill: color.lighten(80%),
    stroke: (paint: color.lighten(40%), thickness: 0.5pt),
  )[
    #text(weight: "medium", size: 9pt, fill: color.darken(30%))[#content]
  ]
}

// Show basic document metadata if front matter exists
#if has-frontmatter [
  #v(1em)
  // title
  #if fm-title != none [
    #align(left)[
      #text(size: 20pt, weight: "medium", fill: rams-black)[#fm-title]
    ]
    #v(0.5em)
  ]
  // subtitle
  #if fm-subtitle != none [
    #align(left)[
      #text(size: 13pt, style: "italic", fill: rams-dark-grey)[#fm-subtitle]
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
    #set text(10pt, fill: rams-dark-grey)
    #for (i, data) in metadata.enumerate() [
      #data
      #if i < metadata.len() - 1 [ • ]
    ]
    #v(0.5em)
  ]

  #if fm-tags != none and tags-list.len() > 0 or fm-participants != none and participants-list.len() > 0 [
    #line(length: 100%, stroke: (paint: rams-brown.lighten(50%), thickness: 1pt))
  ]
  // tags
  #if fm-tags != none and tags-list.len() > 0 [
    #table(
      columns: (20%, 80%),
      align: (right+horizon, left+horizon),
      stroke: none,
      [#smallcaps[
        #if language == "de" {[
          Tags
        ]} else if language == "fr" {[
          Balises
        ]} else {[
          Tags
        ]}
      ]],
      [
        #for (i, tag) in tags-list.enumerate() [
          #badge(tag.trim(), index: tags-list.len()-i)
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
      [#smallcaps[
        #if language == "de" {[
          Teilnehmer
        ]} else if language == "fr" {[
          Participants
        ]} else {[
          Participants
        ]}
      ]],
      [
        #for (i, participants) in participants-list.enumerate() [
          #badge(participants.trim(), index: i)
        ]
      ]
    )
  ]

  // separator
  #line(length: 100%, stroke: (paint: rams-brown.lighten(50%), thickness: 1pt))
]

// table of contents
#if show-toc [
  #set text(fill: rams-dark-grey)
  #show outline.entry: it => {
    set text(size: 10pt)
    it
  }
  #outline(indent: auto)
  #pagebreak()
]

// Enhanced content styling
#show quote: it => {
  set text(style: "italic", fill: rams-dark-grey)
  block(
    fill: rams-orange.lighten(95%),
    inset: (left: 12pt, rest: 8pt),
    radius: 3pt,
    stroke: (left: 3pt + rams-orange, rest: 0.5pt + rams-orange.lighten(60%))
  )[#it]
}

// Table styling with brown accents
#show table: it => {
  set text(size: 10pt)
  block(
    stroke: 1pt + rams-brown.lighten(60%),
    fill: rams-white,
    radius: 3pt,
    clip: true
  )[#it]
}

// Figure captions with orange accent
#set figure(numbering: "1", supplement: [Figure])
#set figure.caption(separator: " — ")
#show figure.caption: it => {
  set text(size: 10pt, style: "italic", fill: rams-orange.darken(20%))
  block(
    fill: rams-orange.lighten(95%),
    inset: 6pt,
    radius: 2pt,
    stroke: 0.5pt + rams-orange.lighten(50%)
  )[#it]
}

#cmarker.render(
  read(filepath),
  scope: (image: (path, alt: none) => image(path, alt: alt)),
  math: mitex
)
