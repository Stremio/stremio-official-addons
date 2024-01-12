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

[v1-npm-link]: https://www.npmjs.com/package/stremio-official-addons
[stremio-official-addons downloads badge]: https://img.shields.io/npm/dm/stremio-official-addons?label=stremio-official-addons%20downloads&link=https%3A%2F%2Fwww.npmjs.com%2Fpackage%2Fstremio-official-addons

[stremio-official-addons version badge]: https://img.shields.io/npm/v/stremio-official-addons?label=stremio-official-addons&link=https%3A%2F%2Fwww.npmjs.com%2Fpackage%2Fstremio-official-addons

[v2-npm-link]: https://www.npmjs.com/package/@stremio/stremio-official-addons
[@stremio/stremio-official-addons downloads badge]: https://img.shields.io/npm/dm/%40stremio/stremio-official-addons?label=%40stremio%2Fstremio-official-addons%20downloads&link=https%3A%2F%2Fwww.npmjs.com%2Fpackage%2F%40stremio%2Fstremio-official-addons

[@stremio/stremio-official-addons version badge]: https://img.shields.io/npm/v/%40stremio/stremio-official-addons?label=%40stremio%2Fstremio-official-addons&link=https%3A%2F%2Fwww.npmjs.com%2Fpackage%2F%40stremio%2Fstremio-official-addons


[v2-crates-io-link]: https://crates.io/crates/stremio-official-addons
[crates version]: https://img.shields.io/crates/v/stremio-official-addons
[crates.io downloads badge]: https://img.shields.io/crates/dv/stremio-official-addons?label=Crates.io%20downloads%20(latest%20version)