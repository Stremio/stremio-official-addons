#!/usr/bin/env node

const client = require('stremio-addon-client')

const ENDPOINTS = [
    "https://v3-cinemeta.strem.io/manifest.json",
    "https://v3-channels.strem.io/manifest.json",
    "https://nfxaddon.strem.io/stremioget/stremio/v1",
    "https://watchhub.strem.io/stremioget/stremio/v1",
    "https://opensubtitles.strem.io/stremio/v1",
    "http://127.0.0.1:11470/local-addon/manifest.json",
]

const col = new client.AddonCollection()

Promise.all(ENDPOINTS.map(url => client.detectFromURL(url)))
.then(function(responses) {
    responses.forEach(function(x, i) {
       if (!x.addon) return
        x.addon.flags = i === 0 ? { official: true, protected: true } : { official: true }
        col.add(x.addon)
    })
})
.then(function() {
    console.log(JSON.stringify(col.save(), null, 4))
})
