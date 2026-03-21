// Brutalist Template
// Raw, bold, stark design with high contrast
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

// Dieter Rams color palette
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

// Extract filename from filepath
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
  margin: (top: 2.5cm, bottom: 2.5cm, left: 2.5cm, right: 2.5cm),
  fill: rams-white
)

// header and footer
#set page(
  header: context(if here().page() >= 2 [
    #set text(size: 9pt, fill: rams-black, font: ("Fira Mono"), weight: "semibold")
    #block(
      fill: rams-black,
      inset: (x: 8pt, y: 4pt),
      radius: 0pt,
      width: 100%
    )[
      #text(fill: rams-white)[#upper(document-title)] #if document-subtitle != none {[#text(fill: rams-white)[ | #upper(document-subtitle)] ]}
    ]
  ]),
  footer: context(if here().page() >= 2 [
    #v(-2pt)
    #line(length: 100%, stroke: 2pt + rams-dark-grey)
    #v(2pt)
    #set text(size: 8pt, fill: rams-black, font: ("Fira Mono"), weight: "semibold")
    #grid(
      columns: (1fr, auto),
      align: (left, right),
      [#if document-author != none [#upper(document-author) |] #document-date.display()],
      [#context counter(page).display("1 / 1", both: true)]
    )
  ])
)

// font & language
#set text(
  font: ("Fira Mono"),
  size: 10pt,
  fill: rams-black,
  lang: language,
  fallback: true
)

// heading
#show heading: set block(above: 1.2em, below: 0.8em)
#set heading(numbering: "1.1")

#show heading.where(level: 1): it => {
  set text(size: 16pt, weight: "black", fill: rams-white)
  set block(above: 1.5em, below: 1.2em)
  if it.numbering != none {
    let num = numbering(it.numbering, ..counter(heading).at(it.location()))
    block(
      fill: rams-black,
      inset: (x: 12pt, y: 8pt),
      radius: 0pt,
      width: 100%
    )[
      #text(weight: "black")[#num] #h(8pt) #upper(it.body)
    ]
  } else {
    block(
      fill: rams-black,
      inset: (x: 12pt, y: 8pt),
      radius: 0pt,
      width: 100%
    )[
      #upper(it.body)
    ]
  }
}

#show heading.where(level: 2): it => {
  set text(size: 13pt, weight: "bold", fill: rams-black)
  set block(above: 1.2em, below: 0.8em)
  if it.numbering != none {
    let num = numbering(it.numbering, ..counter(heading).at(it.location()))
    [#text(weight: "black")[#num] #h(8pt) #upper(it.body)]
  } else {
    [#upper(it.body)]
  }
  v(-4pt)
  line(length: 100%, stroke: 3pt + rams-dark-grey)
}

#show heading.where(level: 3): it => {
  set text(size: 11pt, weight: "bold", fill: rams-black)
  if it.numbering != none {
    let num = numbering(it.numbering, ..counter(heading).at(it.location()))
    [#text(weight: "black")[#num] #h(4pt) #upper(it.body)]
  } else {
    [#upper(it.body)]
  }
  v(-2pt)
  line(length: 60%, stroke: 2pt + rams-dark-grey)
}

// link color
#show link: it => text(fill: rams-green, weight: "bold", it)

// code blocks
#show raw: set text(
  font: ("Iosevka", "DejaVu Sans Mono"),
  fallback: true
)
#show raw.where(block: false): it => {
  box(
    fill: rams-dark-grey,
    inset: (x: 3pt, y: 2pt),
    radius: 0pt,
    stroke: none
  )[
    #text(fill: rams-white, weight: "bold", size: 9pt)[#it]
  ]
}
#show raw.where(block: true): it => {
  set text(size: 9pt, fill: rams-white)
  block(
    fill: rams-black,
    width: 100%,
    inset: 12pt,
    radius: 0pt,
    stroke: 2pt + rams-dark-grey
  )[
    #it
  ]
}

// lists
#set list(indent: 8pt, body-indent: 4pt, marker: ([■], [▪], [▫]))
#show list: it => {
  set text(fill: rams-black)
  it
}

// emphasis
#show emph: set text(style: "italic", fill: rams-dark-grey, weight: "bold")
#show strong: set text(weight: "black", fill: rams-black)

// quotes
#show quote: it => {
  set text(fill: rams-black, weight: "bold")
  block(
    fill: rams-light-grey,
    inset: (left: 12pt, rest: 8pt),
    radius: 0pt,
    stroke: (left: 4pt + rams-red, rest: 2pt + rams-dark-grey)
  )[#it]
}

#show table: it => {
  set text(size: 9pt, weight: "bold")
  set table(
    stroke: 2pt + rams-black,
    fill: rams-white,
  )
  it
}

// figure captions
#set figure(numbering: "1", supplement: [FIG])
#set figure.caption(separator: " - ")
#show figure.caption: it => {
  set text(size: 9pt, weight: "bold", fill: rams-white)
  block(
    fill: rams-black,
    inset: 6pt,
    width: 100%,
    radius: 0pt
  )[
    #it
  ]
}

// badge function
#let badge(content) = {
  box(
    fill: rams-orange,
    inset: (x: 6pt, y: 3pt),
    radius: 0pt,
    stroke: 2pt + rams-black
  )[
    #text(weight: "black", size: 8pt, fill: rams-white)[#upper(content)]
  ]
}

// Show basic document metadata if front matter exists
#if has-frontmatter [
  // title
  #if fm-title != none [
    #align(left)[
      #block(
        fill: rams-black,
        inset: (x: 16pt, y: 12pt),
        radius: 0pt,
        width: 100%,
        stroke: none
      )[
        #text(size: 22pt, weight: "black", fill: rams-white)[#upper(fm-title)]
      ]
    ]
    #v(0.8em)
  ]
  // subtitle
  #if fm-subtitle != none [
    #align(left)[
      #block(
        fill: rams-dark-grey,
        inset: (x: 12pt, y: 6pt),
        radius: 0pt,
        stroke: 2pt + rams-black
      )[
        #text(size: 14pt, weight: "bold", fill: rams-white)[#upper(fm-subtitle)]
      ]
    ]
    #v(0.8em)
  ]
  // logo
  #if fm-logo != none {[#figure(image(fm-logo,width:4cm))]}
  // metadata (author, date, version)
  #let metadata = ()
  #if document-author != none { metadata.push([#upper(document-author)]) }
  #if document-date != none { metadata.push([#document-date.display()]) }
  #if fm-version != none { metadata.push([#fm-version]) }
  #if metadata.len() > 0 [
    #set text(size: 10pt, fill: rams-black, weight: "bold")
    #grid(
      columns: metadata.len(),
      column-gutter: 12pt,
      ..metadata.map(data => block(
        fill: rams-light-grey,
        inset: 6pt,
        stroke: 1pt + rams-dark-grey,
        radius: 0pt
      )[#data])
    )
    #v(0.8em)
  ]

  #if fm-tags != none and tags-list.len() > 0 or fm-participants != none and participants-list.len() > 0 [
    #line(length: 100%, stroke: 2pt + rams-black)
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
      align: (right, left),
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

  // separator
  #line(length: 100%, stroke: 2pt + rams-black)
]

// table of contents
#if show-toc [
  #text(size: 16pt, weight: "black", fill: rams-black)[CONTENTS]
  #v(0.5em)
  #line(length: 100%, stroke: 2pt + rams-black)
  #v(0.5em)
  #set text(weight: "semibold", fill: rams-black)
  #outline(indent: auto)
  #pagebreak()
]

#cmarker.render(
  read(filepath),
  scope: (image: (path, alt: none) => image(path, alt: alt)),
  math: mitex
)
