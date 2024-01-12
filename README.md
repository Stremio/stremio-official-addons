# stremio-official-addons
[![npm][stremio-official-addons version badge]][v1-npm-link] ![downloads][stremio-official-addons downloads badge]

[![npm][@stremio/stremio-official-addons version badge]][v2-npm-link] ![downloads][@stremio/stremio-official-addons downloads badge] [![crates.io version badge][crates version]][v2-crates-io-link] ![Crates.io Downloads (latest version)][crates.io downloads badge]

All the stremio official add-ons, in `AddonCollection.load()` format

The `index.js` file is meant to be generated from the output of `AddonCollection.save()`


## Changing the official add-ons

To change what's contained in this module, you have to change [`scripts/gen.js`](./scripts/gen.js) for **version 1** and [`official-addons-v2/scripts/build.js`](./official-addons-v2/scripts/build.js) for **version 2**.

Besides changing this module, you have to update all it's dependant packages, most notably:

* stremio-api-workers: this is the **most important one**, since it will affect `addonsofficialcollection.json` and therefore the addonCollection API, and therefore all running apps should respect it ([`stremio-official-addons@1`][v1-npm-link] package)

* stremio: the desktop/web app ([`stremio-official-addons@1` package][v1-npm-link])

* stremio-rn: the react native app ([`stremio-official-addons@1` package][v1-npm-link])

* stremio-web: the new desktop/web app ([`@stremio/stremio-official-addons@2` package][v2-npm-link])

* stremio-core: the core for most applications including web, android tv, android, etc. ([`stremio-official-addons@2` crate][v2-crates-io-link])

* stremio-api-legacy-shim: this one does not *depend on* this module, and does not include it, but it contains a separate list of official add-ons in `addons/official.js` that is respected by all apps using the legacy endpoints **ARCHIVED**

# Release new version

## Version 1.XX.XX
To release a new version for v1 **npm** package you need to:

1. Bump [`package.json`](./package.json)
1.1. Make sure to update [`package-lock.json`](./package-lock.json):
`npm i --package-lock-only`
2. Make a new tag

`git tag -a v1.XX.XX -m "Tag for releases v1.XX.XX"`

3. Push it to the repo

`git push -u origin v1.XX.XX`

4. Create a [new release](https://github.com/Stremio/stremio-official-addons/releases) using the version as a title, e.g. `v1.5.6` and automatically generate changes from previous tag.

5. The [`publish` workflow](./.github/workflows/publish.yml) will run the `v1` job and will:
   1. Build [`stremio-official-addons@1`][v1-npm-link] package
   2. Publish the [package on npm][v1-npm-link]

## Version 2.XX.XX

To release a new version for v2 **npm** and **crate** packages you need to:
1. Bump [`official-addons-v2/package.json`](./official-addons-v2/package.json)
1.1. Make sure to update [`package-lock.json`](./package-lock.json):

`npm i --package-lock-only`

2. Bump [`official-addons-v2/Cargo.toml`](./official-addons-v2/Cargo.toml)
2.1. Make sure to update [`Cargo.lock`](./Cargo.lock):

`cargo update -p stremio-official-addons`

3. Make a new tag

`git tag -a v2.XX.XX -m "Tag for releases v2.XX.XX"`

4. Push it to the repo

`git push -u origin v2.XX.XX`

5. Create a [new release](https://github.com/Stremio/stremio-official-addons/releases) using the version as a title, e.g. `v2.0.12` and automatically generate changes from previous tag.

6. The [`publish` workflow](./.github/workflows/publish.yml) will run the `v2` job and will:
   1. Build [`@stremio/stremio-official-addons@2`][v2-npm-link] package
   2. Publish the [package on npm][v2-npm-link]
   3. Publish the [crate on crates.io][v2-crates-io-link]

# License

Both packages are licensed under the MIT license:
- [`stremio-official-addons` v1 MIT license](./LICENSE.md)
- [`stremio-official-addons` v2 MIT license](./official-addons-v2/LICENSE.md)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the project by you, shall be licensed as MIT,
without any additional terms or conditions.

[v1-npm-link]: https://www.npmjs.com/package/stremio-official-addons
[stremio-official-addons downloads badge]: https://img.shields.io/npm/dm/stremio-official-addons?label=stremio-official-addons%20downloads&link=https%3A%2F%2Fwww.npmjs.com%2Fpackage%2Fstremio-official-addons

[stremio-official-addons version badge]: https://img.shields.io/npm/v/stremio-official-addons?label=stremio-official-addons&link=https%3A%2F%2Fwww.npmjs.com%2Fpackage%2Fstremio-official-addons

[v2-npm-link]: https://www.npmjs.com/package/@stremio/stremio-official-addons
[@stremio/stremio-official-addons downloads badge]: https://img.shields.io/npm/dm/%40stremio/stremio-official-addons?label=%40stremio%2Fstremio-official-addons%20downloads&link=https%3A%2F%2Fwww.npmjs.com%2Fpackage%2F%40stremio%2Fstremio-official-addons

[@stremio/stremio-official-addons version badge]: https://img.shields.io/npm/v/%40stremio/stremio-official-addons?label=%40stremio%2Fstremio-official-addons&link=https%3A%2F%2Fwww.npmjs.com%2Fpackage%2F%40stremio%2Fstremio-official-addons


[v2-crates-io-link]: https://crates.io/crates/stremio-official-addons
[crates version]: https://img.shields.io/crates/v/stremio-official-addons
[crates.io downloads badge]: https://img.shields.io/crates/dv/stremio-official-addons?label=Crates.io%20downloads%20(latest%20version)