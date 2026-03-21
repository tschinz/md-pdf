# Changelog

All notable changes to this project will be documented in this file.

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
