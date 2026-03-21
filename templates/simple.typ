// Simple Template
// Professional with headers/footers
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
  // Try to parse the date string
  let date-str = fm-date
  if date-str.len() == 10 and date-str.contains("-") {
    // Format: YYYY-MM-DD
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
#set page(margin: (top:3cm, bottom:3cm, left:3cm, right:2.5cm))

// header and footer
#set page(
  header: context(if here().page() >=2 [
    #table(
      columns: (80%, 20%),
      stroke: none,
      inset: -0.5em,
      align: (x, y) => (left+bottom, right+top).at(x),
      [#smallcaps[#document-title] #if document-subtitle != none {[| #smallcaps[#document-subtitle] ]}],
      [#if fm-logo != none {[#v(1.2cm)#image(fm-logo,width:2cm)]}]
    )
    ]),
  footer: context( if here().page() >=2 [
      #set text(10pt)
      #if document-author != none {document-author} #h(1fr) #document-date.display() #h(1fr) #context counter(page).display("1 / 1", both: true)
  ]),
)

// font & language
#set text(
  font: (
    "Libertinus Serif",
  ),
  fallback: true,
  lang: language
)

// heading
#show heading: set block(above: 1.2em, below: 1.2em)
#set heading(numbering: "1.1")

// link color
#show link: it => text(fill:blue, it)

// code blocks
#show raw: set text(
  font: (
  "DejaVu Sans Mono",
  ),
fallback: true,)
#show raw.where(block: false): set text(weight: "semibold")
#show raw.where(block: true): set text(size: 8pt)

// badge function
#let badge(content) = {
  let color = rgb("888888")
  let textcolor = rgb("222222")
  box(
    inset: (x: 3pt, y: 2pt),
    radius: 4pt,
    fill: color.lighten(70%),
    stroke: (paint: color, thickness: 0.5pt),
  )[
    #text(weight: "bold", size: 8pt, fill:textcolor)[#content]
  ]
}

// Show basic document metadata if front matter exists
#if has-frontmatter [
  // title
  #if fm-title != none [
    #align(center)[
      #text(size: 18pt, weight: "bold")[#fm-title]
    ]
    #v(0.3em)
  ]
  // subtitle
  #if fm-subtitle != none [
    #align(center)[
      #text(size: 14pt, style: "italic")[#fm-subtitle]
    ]
    #v(0.3em)
  ]
  // logo
  #if fm-logo != none {[#figure(image(fm-logo,width:4cm))]}
  // metadata (author, date, version)
  #let metadata = ()
  #if document-author != none { metadata.push(document-author) }
  #if document-date != none { metadata.push(document-date.display()) }
  #if fm-version != none { metadata.push(fm-version) }
  #align(center)[
    #for (i, data) in metadata.enumerate() [
      #data
      #if i < metadata.len() - 1 [ \- ]
    ]
  ]
  #if fm-tags != none and tags-list.len() > 0 or fm-participants != none and participants-list.len() > 0 [
    #align(center)[#line(length: 90%, stroke: 0.5pt)]
  ]
  // tags
  #if fm-tags != none and tags-list.len() > 0 [
    #table(
      columns: (20%, 80%),
      align: (right, left),
      stroke: none,
      [#smallcaps[
        #if language == "de" {[
          *Tags*
        ]} else if language == "fr" {[
          *Balises*
        ]} else {[
          *Tags*
        ]}
      ]],
      [
        #for (i, tag) in tags-list.enumerate() [
          #badge(tag.trim())
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
          *Teilnehmer*
        ]} else if language == "fr" {[
          *Participants*
        ]} else {[
          *Participants*
        ]}
      ]],
      [
        #for (i, participants) in participants-list.enumerate() [
          #badge(participants.trim())
        ]
      ]
    )
  ]

  #align(center)[#line(length: 90%, stroke: 0.5pt)]
]
// table of contents
#if show-toc [
  #outline()
  #pagebreak()
]

#cmarker.render(
  read(filepath),
  scope: (image: (path, alt: none) => image(path, alt: alt)),
  math: mitex
)
