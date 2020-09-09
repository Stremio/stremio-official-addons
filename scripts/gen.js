#!/usr/bin/env node

const client = require('stremio-addon-client')

const ENDPOINTS = [
    "https://v4-cinemeta.strem.io/manifest.json"
    // "https://v3-channels.strem.io/manifest.json",
    // "https://watchhub.strem.io/manifest.json",
    // "https://caching.stremio.net/publicdomainmovies.now.sh/manifest.json",
    // "https://opensubtitles.strem.io/stremio/v1",
    // "http://127.0.0.1:11470/local-addon/manifest.json",
]

const col = new client.AddonCollection()

Promise.all(ENDPOINTS.map(url => client.detectFromURL(url)))
.then(function(responses) {
    responses.forEach(function(x, i) {
       if (!x.addon) return
	if (JSON.stringify(x.addon.manifest).length > 8192) throw 'manifest bigger than 8kb - aborting!'
        x.addon.flags = isProtected(x, i) ? { official: true, protected: true } : { official: true }
        col.add(x.addon)
    })
})
.then(function() {
    console.log(JSON.stringify(col.save(), null, 4))
})

function isProtected(x, i) {
	// cinemeta
	if (i === 0) return true

	// local
	if (x.addon.manifest.id.match('local')) return true
}
