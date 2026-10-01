# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0] - 2026-10-01

### 🚀 Features

- Embed the Typst compiler ([d967fe8](https://github.com/tschinz/md-pdf/commit/d967fe8c2c627ba94c685f389194446c9927f698) - zas)

### 🐛 Bug Fixes

- *(justfile)* Regenerate the SBOM before uploading it ([467a6ef](https://github.com/tschinz/md-pdf/commit/467a6ef8312aea883c6f1e89c25b5c1a4cba4014) - zas)

### 💼 Other

- Strip symbols from release binaries ([9218432](https://github.com/tschinz/md-pdf/commit/92184328329df9acc122653acb388ea10f6c6ba9) - zas)
- Exclude private template example sources from the crate ([4c2e05e](https://github.com/tschinz/md-pdf/commit/4c2e05e47456e28c31f48ec94b08b5add71b0715) - zas)

### 🚜 Refactoring

- *(link-checker)* Replace reqwest with ureq ([4cbdf96](https://github.com/tschinz/md-pdf/commit/4cbdf969fac86c055864b49f2349f98214422b5f) - zas)
- *(templates)* Remove dead .temp.md filename branch ([b33889c](https://github.com/tschinz/md-pdf/commit/b33889c43978a7d850969ada7e886067253e7f66) - zas)

### 📚 Documentation

- Drop the Typst installation prerequisite ([fe96a9a](https://github.com/tschinz/md-pdf/commit/fe96a9a240eb5d4fd17bcc14093fbfc25c0b9c23) - zas)
- *(readme)* Unwrap hard-wrapped paragraphs ([8596918](https://github.com/tschinz/md-pdf/commit/8596918f51e7411e936670be90d0c0eb834dd679) - zas)
- Install with cargo install --locked ([2cdf1a7](https://github.com/tschinz/md-pdf/commit/2cdf1a7516fbc6c3eaeacfce1d5c3be489bd78dc) - zas)

### ⚙️ Miscellaneous Tasks

- *(deps)* Update dirs to 6.0 ([0a3a96c](https://github.com/tschinz/md-pdf/commit/0a3a96cb55b4d06955617753ca36821552be1e50) - zas)
- *(justfile)* Remove typst-cli install and check recipes ([d5188e6](https://github.com/tschinz/md-pdf/commit/d5188e6560719840029350f68b70a7a85d3465f3) - zas)
- *(release)* Bump version to 0.3.0 ([45847e5](https://github.com/tschinz/md-pdf/commit/45847e5e8025dd451cae431189b90b139c736c35) - zas)
- *(examples)* Regenerate example PDFs ([58d2933](https://github.com/tschinz/md-pdf/commit/58d2933f6482eb985796b18d04448fea85054fee) - zas)
- *(changelog)* Generate changelog for 0.3.0 ([39be0f1](https://github.com/tschinz/md-pdf/commit/39be0f153a41a685bf40dc0695300e21766035b5) - zas)
- *(deps)* Update compatible Rust dependencies ([9f0f74a](https://github.com/tschinz/md-pdf/commit/9f0f74ad62bc407d1b6774782516526084801f56) - zas)
- *(deps)* Upgrade ron to 0.12, dirs to 7, notify to 8 and opener to 0.9 ([bd8f035](https://github.com/tschinz/md-pdf/commit/bd8f03573884ee5768782fc3354bc8135ff39702) - zas)
- *(templates)* Update mitex to 0.2.7 ([360b626](https://github.com/tschinz/md-pdf/commit/360b62645131239af7b1aec5d64d99e4313c78a9) - zas)
- *(justfile)* Combine rustfmt and clippy into a lint recipe ([e1f627a](https://github.com/tschinz/md-pdf/commit/e1f627a44fec2298d8295b08394150f5fc364e22) - zas)
- *(justfile)* Install releases with --locked ([c19d634](https://github.com/tschinz/md-pdf/commit/c19d634a078bc45d3fa3712e8cfdbb6bd3916a1a) - zas)
- *(examples)* Regenerate the guide PDF ([83d99f8](https://github.com/tschinz/md-pdf/commit/83d99f8130eae428c358e537aec8fdd132d77afc) - zas)


**Full Changelog**: [v0.2.0...0.3.0](https://github.com/tschinz/md-pdf/compare/v0.2.0...0.3.0)

## [0.2.0] - 2026-09-24

### 🚀 Features

- *(frontmatter)* [**breaking**] Adopt OKF-canonical field names with legacy aliases ([8c8ab04](https://github.com/tschinz/md-pdf/commit/8c8ab04e0bdaf729ef825cc93eb3660569b5dc2a) - zas)
- *(templates)* Add document type label and adopt OKF field names ([21d7ce7](https://github.com/tschinz/md-pdf/commit/21d7ce718ffd1d59866e2052eeb32d1ef427dc1a) - zas)

### 📚 Documentation

- *(readme)* Document OKF front matter compatibility ([f7f893b](https://github.com/tschinz/md-pdf/commit/f7f893b697497e1056fdfd5315abf9e72a8af80f) - zas)
- *(examples)* Use OKF field names and add type field ([0b364e3](https://github.com/tschinz/md-pdf/commit/0b364e381d806b6b6822b06a5a05969be5d515de) - zas)
- *(examples)* Add letter and professional examples ([f2d0733](https://github.com/tschinz/md-pdf/commit/f2d073319aa31252a217eaf5ea65224f8483b0de) - zas)

### ⚙️ Miscellaneous Tasks

- *(examples)* Regenerate example PDFs ([7d6d763](https://github.com/tschinz/md-pdf/commit/7d6d763744d8318ee55bbfb51362f6157cf7569c) - zas)
- Keep private letter/professional templates out of git and crate ([b21d8b8](https://github.com/tschinz/md-pdf/commit/b21d8b897c0eb2b346b4bf4e65bafbc72ec2dc58) - zas)
- *(release)* Bump version to 0.2.0 ([eccb9e4](https://github.com/tschinz/md-pdf/commit/eccb9e4bce311d4bb2d927b701e6c778ff2767fc) - zas)


**Full Changelog**: [v0.1.3...0.2.0](https://github.com/tschinz/md-pdf/compare/v0.1.3...0.2.0)

## [0.1.3] - 2026-09-19

### 🚀 Features

- *(templates)* Size and align images via HTML <img> tag ([b126655](https://github.com/tschinz/md-pdf/commit/b126655780638a1db397cea20b6d3d60e63a62e7) - zas)

### 🐛 Bug Fixes

- *(deps)* Upgrade reqwest to 0.12 to resolve security advisories ([cd407c2](https://github.com/tschinz/md-pdf/commit/cd407c25adc2c64042c14264bc04f68b41fe146f) - zas)

### 📚 Documentation

- Document image sizing and alignment ([6b6d813](https://github.com/tschinz/md-pdf/commit/6b6d813996a0a111e541e1f6522cc27da85c187c) - zas)

### ⚙️ Miscellaneous Tasks

- *(examples)* Regenerate example PDFs ([bc6565a](https://github.com/tschinz/md-pdf/commit/bc6565a650c3c2f763f62d983d3ca140993df35c) - zas)
- *(release)* Bump version to 0.1.3 ([8dc9781](https://github.com/tschinz/md-pdf/commit/8dc97815fa1212eb22280b3e65bcd4c6d0cd31bc) - zas)


**Full Changelog**: [v0.1.2...0.1.3](https://github.com/tschinz/md-pdf/compare/v0.1.2...0.1.3)

## [0.1.2] - 2026-03-21

### 🚀 Features

- *(templates)* Add templaes as external files instead of embed in the code ([b0bd399](https://github.com/tschinz/md-pdf/commit/b0bd399aaf180c1bce384a462f5468f137627b87) - zas)
- *(refresh)* Refresh outdates templates ([a395c82](https://github.com/tschinz/md-pdf/commit/a395c82293a64ec960428e2ec143819cc2d33e86) - zas)
- *(build)* Add templates dynamically with the build.rs as module embedded_templates.rs ([1be194d](https://github.com/tschinz/md-pdf/commit/1be194da8cef543020fb037aa81f3bbf95761e04) - zas)

### 🐛 Bug Fixes

- *(default_author)* Md-pdf default author in config file ([5f7f8ac](https://github.com/tschinz/md-pdf/commit/5f7f8ac4580edbaee7aced21cf4fc5a0307c8c3a) - zas)
- *(lint)* Clippy and rustfmt ([f6779ff](https://github.com/tschinz/md-pdf/commit/f6779ff2fe7fdc43d7dbd8dafa6c7c2c000943cd) - zas)

### 📚 Documentation

- Updated readme and comprehensive-guide ([4fdb725](https://github.com/tschinz/md-pdf/commit/4fdb7253da61bafad1e0fbd246c416b117e8a18f) - zas)

### ⚙️ Miscellaneous Tasks

- *(test)* Add more test for raw-md ([8e59b75](https://github.com/tschinz/md-pdf/commit/8e59b7593dc0f5cbf72464d8e6305cd8e18e42aa) - zas)
- *(version)* Bump version to 0.1.2 ([4219b9d](https://github.com/tschinz/md-pdf/commit/4219b9d2d52f00a680946d77b107f17ca4d217a9) - zas)


**Full Changelog**: [v0.1.1...0.1.2](https://github.com/tschinz/md-pdf/compare/v0.1.1...0.1.2)

## [0.1.1] - 2026-03-12

### 🐛 Bug Fixes

- *(templates)* Create default config and templates if none exist ([08ca2b0](https://github.com/tschinz/md-pdf/commit/08ca2b054be87536a2f755e66744600a9c877abc) - zas)
- *(fmt)* Run rust fmt ([a4ebc60](https://github.com/tschinz/md-pdf/commit/a4ebc6058ac1c08696a921d5036028e3faafefac) - zas)

### 📚 Documentation

- *(readme)* Add install option crates.io ([5539049](https://github.com/tschinz/md-pdf/commit/5539049e0d082a221fab60b273d0bb8ef8f08c7f) - zas)

### 🧪 Testing

- *(raw-md)* Add new testfile raw-md ([5651405](https://github.com/tschinz/md-pdf/commit/5651405a141712e0a7ec8322ecb96753e61eb1c1) - zas)

### ⚙️ Miscellaneous Tasks

- *(readme)* Extend readme with badge and tianji tracker ([7e198a3](https://github.com/tschinz/md-pdf/commit/7e198a3396f280ef64f9e293ea3c71efd950f214) - tschinz)
- *(cliff)* Change cliff config ([ee1273b](https://github.com/tschinz/md-pdf/commit/ee1273b9b3e040c3998ad4c86c8d5b804ff3721d) - zas)
- *(just)* Add default value for watch recipe ([89eeee8](https://github.com/tschinz/md-pdf/commit/89eeee82780b9e0d0e1508b8bd4a78dbabd6e03a) - zas)
- *(just)* Add publish-check recpie ([bfb4b68](https://github.com/tschinz/md-pdf/commit/bfb4b687f64dbec757e07d6c8b0d31b364129c51) - zas)

### ◀️ Revert

- Remove example file ([0248127](https://github.com/tschinz/md-pdf/commit/0248127f775f250bc83b7f09e7dc13635201f771) - zas)


**Full Changelog**: [v0.1.0...0.1.1](https://github.com/tschinz/md-pdf/compare/v0.1.0...0.1.1)

# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

---
## [0.1.0](https://github.com/tschinz/md-pdf/compare/0.0.2..0.1.0) - 2026-02-09

### 🚀 Features

- **(open)** add auto-open feature flag - ([e6e9b57](https://github.com/tschinz/md-pdf/commits/e6e9b57dee125bb398a8e4225e11d596a86fd3da)) - zas
- **(sbom)** add sbom in justfile and release workflow - ([3e347d6](https://github.com/tschinz/md-pdf/commits/3e347d64225e0410ad9ba4ae45b3d83e374d5f99)) - zas
- **(template)** add darko template - ([f99869f](https://github.com/tschinz/md-pdf/commits/f99869f38adcda9d10b9da1e5d9a5b265de0dfca)) - zas

### 🐛 Bug Fixes

- **(templates)** beautify and improve default templates, support for additional frontmatter fields - ([667576d](https://github.com/tschinz/md-pdf/commits/667576d6f0c06b5e94975c8a6ddc5e661af6a25e)) - zas
- clippy issues and test config issues - ([f555bdc](https://github.com/tschinz/md-pdf/commits/f555bdcc4ee9136fb7ac3e3c4696017b9b7b17c8)) - zas

### 📚 Documentation

- **(example)** optimize the example with all template supported frontmatter fields - ([34dc875](https://github.com/tschinz/md-pdf/commits/34dc875d01e812c1007d6c54d458d23b4cda5a80)) - zas
- **(guide)** add images of the default templates and minor changes - ([27939e0](https://github.com/tschinz/md-pdf/commits/27939e08be6ae9615d12550a78f421434e22ed00)) - zas
- **(guide)** add darko template - ([5498b7c](https://github.com/tschinz/md-pdf/commits/5498b7c86a90e60362c70a49039df4d5fac1d3d1)) - zas
- **(rustdoc)** add rustdoc (AI-generated) - ([7606b3a](https://github.com/tschinz/md-pdf/commits/7606b3aa8f02df53bccae5999dce3c812cdbad4d)) - zas

### ⚙️ Miscellaneous Tasks

- **(ci)** add github workflow for ci and release - ([a7fb41d](https://github.com/tschinz/md-pdf/commits/a7fb41d3199e1ca6960269c6805d158d17817bf5)) - zas
- **(creates)** upgrades all crates - ([5df47f0](https://github.com/tschinz/md-pdf/commits/5df47f006dc339a070f27b92c1517e5ecc910d72)) - zas
- **(release)** add trivy test and sbom upload - ([a5bb0fa](https://github.com/tschinz/md-pdf/commits/a5bb0fa4edaca7b1ea0dae4a3baee32e3b228efb)) - zas
- **(release)** update changelog for 0.1.0 - ([0cb542a](https://github.com/tschinz/md-pdf/commits/0cb542a4614359ee4ba12d47621c16fd0774d6ba)) - zas
- bump rust edition add and smaller fixes, prepare for 0.1.0 release - ([4fb21a2](https://github.com/tschinz/md-pdf/commits/4fb21a297d35f77df316553bc0265324e199a459)) - zas

---
## [0.0.2](https://github.com/tschinz/md-pdf/compare/0.0.1..0.0.2) - 2026-01-26

### 🚀 Features

- **(example)** add starfleet 1-2 page example - ([6f36357](https://github.com/tschinz/md-pdf/commits/6f363571b3ff9b72093777d1ff3896eb9ba41026)) - zas
- **(templates)** new brutalist and playful template - ([278dac8](https://github.com/tschinz/md-pdf/commits/278dac80d7eea3d4bc5c19a060258ed9fac08ed6)) - zas
- add support for custom frontmatter fields as sys.inputs - ([7ffc1f5](https://github.com/tschinz/md-pdf/commits/7ffc1f53ebe794fbeea2bb9f6cb3f91398865372)) - zas

### 🐛 Bug Fixes

- **(readme)** image resize - ([2da1202](https://github.com/tschinz/md-pdf/commits/2da12022e55ba4c1fef0d37900626c93c18d17a9)) - zas
- **(templates)** fix minor issues with the none and simple template - ([bcd5f76](https://github.com/tschinz/md-pdf/commits/bcd5f768c4b3515b8a2fe69984e7561c76cb4d18)) - zas

### 💼 Other

-  [**breaking**]prepare package for crates.io publication - ([40d0c27](https://github.com/tschinz/md-pdf/commits/40d0c279fbfde24a0a7a4dedbb6a19e48bac4dae)) - zas
-  [**breaking**]prepare package for crates.io publication - ([40d0c27](https://github.com/tschinz/md-pdf/commits/40d0c279fbfde24a0a7a4dedbb6a19e48bac4dae)) - zas
-  [**breaking**]prepare package for crates.io publication - ([40d0c27](https://github.com/tschinz/md-pdf/commits/40d0c279fbfde24a0a7a4dedbb6a19e48bac4dae)) - zas
-  [**breaking**]prepare package for crates.io publication - ([40d0c27](https://github.com/tschinz/md-pdf/commits/40d0c279fbfde24a0a7a4dedbb6a19e48bac4dae)) - zas
- add support for custom frontmatter fields as sys.inputs - ([7ffc1f5](https://github.com/tschinz/md-pdf/commits/7ffc1f53ebe794fbeea2bb9f6cb3f91398865372)) - zas
- add support for custom frontmatter fields as sys.inputs - ([7ffc1f5](https://github.com/tschinz/md-pdf/commits/7ffc1f53ebe794fbeea2bb9f6cb3f91398865372)) - zas
- add support for custom frontmatter fields as sys.inputs - ([7ffc1f5](https://github.com/tschinz/md-pdf/commits/7ffc1f53ebe794fbeea2bb9f6cb3f91398865372)) - zas
- add support for custom frontmatter fields as sys.inputs - ([7ffc1f5](https://github.com/tschinz/md-pdf/commits/7ffc1f53ebe794fbeea2bb9f6cb3f91398865372)) - zas
- add support for custom frontmatter fields as sys.inputs - ([7ffc1f5](https://github.com/tschinz/md-pdf/commits/7ffc1f53ebe794fbeea2bb9f6cb3f91398865372)) - zas
- Custom frontmatter fields not available in templates - ([7ffc1f5](https://github.com/tschinz/md-pdf/commits/7ffc1f53ebe794fbeea2bb9f6cb3f91398865372)) - zas

### 📚 Documentation

- update readme and comprehensive guide with the new feature and add logo - ([e0788ee](https://github.com/tschinz/md-pdf/commits/e0788ee28779e8af2592681bcb4531e7159874e4)) - zas

### ⚙️ Miscellaneous Tasks

- **(guide)** cleanup - ([a0b1da9](https://github.com/tschinz/md-pdf/commits/a0b1da91fbc006520626702f53f18818cbe73a4a)) - zas
- **(version)** prepare for 0.0.2 prerelease - ([bf9de31](https://github.com/tschinz/md-pdf/commits/bf9de31916d20b9d6379726833c7235dbf93becf)) - zas
- prepare package for crates.io publication - ([40d0c27](https://github.com/tschinz/md-pdf/commits/40d0c279fbfde24a0a7a4dedbb6a19e48bac4dae)) - zas

---
## [0.0.1] - 2026-01-23

### 🚀 Features

- pdf conversion - ([bbd6f55](https://github.com/tschinz/md-pdf/commits/bbd6f55293446baadfabd7dfa1eecdb4a231a102)) - zas
- typst templates simple and none - ([bbd6f55](https://github.com/tschinz/md-pdf/commits/bbd6f55293446baadfabd7dfa1eecdb4a231a102)) - zas
- configuration and auto-configuration - ([bbd6f55](https://github.com/tschinz/md-pdf/commits/bbd6f55293446baadfabd7dfa1eecdb4a231a102)) - zas
- watch mode - ([bbd6f55](https://github.com/tschinz/md-pdf/commits/bbd6f55293446baadfabd7dfa1eecdb4a231a102)) - zas
- link checker - ([bbd6f55](https://github.com/tschinz/md-pdf/commits/bbd6f55293446baadfabd7dfa1eecdb4a231a102)) - zas
- frontmatter support - ([bbd6f55](https://github.com/tschinz/md-pdf/commits/bbd6f55293446baadfabd7dfa1eecdb4a231a102)) - zas

### 💼 Other

- Initial commit - ([ae31527](https://github.com/tschinz/md-pdf/commits/ae315272961b39fd423e0e17c29e650702064884)) - tschinz

### 📚 Documentation

- **(guide)** generate pdf of the comprehensive-guide - ([9833ece](https://github.com/tschinz/md-pdf/commits/9833ece9b97da04d11fa410b942843e5278fc711)) - zas
- comprehensive-guide - ([bbd6f55](https://github.com/tschinz/md-pdf/commits/bbd6f55293446baadfabd7dfa1eecdb4a231a102)) - zas

### ⚙️ Miscellaneous Tasks

- **(release)** prepare for release 0.0.1 - ([e574638](https://github.com/tschinz/md-pdf/commits/e57463860bbc694fcdae3969108d734c4c5fba20)) - zas
- cliff configuration - ([bbd6f55](https://github.com/tschinz/md-pdf/commits/bbd6f55293446baadfabd7dfa1eecdb4a231a102)) - zas
- rustfmt configuration - ([bbd6f55](https://github.com/tschinz/md-pdf/commits/bbd6f55293446baadfabd7dfa1eecdb4a231a102)) - zas
- rustfmt formatting - ([6ea1143](https://github.com/tschinz/md-pdf/commits/6ea11436deac286a0d40cc880eab944b409d93b4)) - zas

<!-- generated by git-cliff -->
