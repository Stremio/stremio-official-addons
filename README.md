# stremio-official-addons

All the stremio official add-ons, in `AddonCollection.load()` format

The `index.js` file is meant to be generated from the output of `AddonCollection.save()`


## Update

To re-generate `index.json` you can do `scripts/gen.js > index.json`


## Changing the official add-ons

To change what's contained in this module, you have to change `scripts/gen.js`

Besides changing this module, you have to update all it's dependant packages, most notably:

* stremio-api-workers: this is the **most important one**, since it will affect `addonsofficialcollection.json` and therefore the addonCollection API, and therefore all running apps should respect it

* stremio: the desktop/web app

* stremio-rn: the react native app

* stremio-web: the new desktop/web app

* stremio-api-legacy-shim: this one does not *depend on* this module, and does not include it, but it contains a separate list of official add-ons in `addons/official.js` that is respected by all apps using the legacy endpoints 
