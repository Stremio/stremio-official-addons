#!/usr/bin/env node

const client = require('stremio-addon-client');
const AddonClient = require('stremio-addon-client/lib/AddonClient');
const transports = require('stremio-addon-client/lib/transports');
const localAddonManifest = require('stremio-local-addon/lib/manifestNoCatalogs');

const PROTECTED_URLS = [
    'https://v3-cinemeta.strem.io/manifest.json',
    'http://127.0.0.1:11470/local-addon/manifest.json',
];
const ENDPOINTS = [
    "https://v3-cinemeta.strem.io/manifest.json",
    "https://v3-channels.strem.io/manifest.json",
    "https://watchhub.strem.io/manifest.json",
    "https://caching.stremio.net/publicdomainmovies.now.sh/manifest.json",
    "https://opensubtitles-v3.strem.io/manifest.json",
    "https://opensubtitles.strem.io/stremio/v1",
    "http://127.0.0.1:11470/local-addon/manifest.json",
]

const col = new client.AddonCollection()

Promise.all(ENDPOINTS.map((transportUrl) => {
    if (transportUrl === 'http://127.0.0.1:11470/local-addon/manifest.json') {
        return Promise.resolve({ addon: new AddonClient(localAddonManifest, new transports.http(transportUrl), { official: true, protected: true }), transportUrl });
    }

    return client.detectFromURL(transportUrl);
}))
    .then(function (responses) {
        responses.forEach(function (response) {
            if (!response.addon) return
            let descriptor = response.addon.toDescriptor();

            if (JSON.stringify(descriptor.manifest).length > 8192) throw 'manifest bigger than 8kb - aborting!'
            let isProtected = PROTECTED_URLS.includes(descriptor.transportUrl);

            col.add(new AddonClient(descriptor.manifest, new transports.http(descriptor.transportUrl), { official: true, protected: isProtected }))
        })
    })
    .then(function () {
        console.log(JSON.stringify(col.save(), null, 4))
    })
