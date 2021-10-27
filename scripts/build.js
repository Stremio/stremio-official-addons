#!/usr/bin/env node

const fs = require('fs');
const fetch = require('node-fetch');
const validator = require('@stremio/stremio-core-validator');
const localAddonManifest = require('stremio-local-addon/lib/manifestNoCatalogs');
const legacyManifestMapper = require('stremio-addon-client/lib/transports/legacy/mapper');

const LEGACY_REQUEST_PARAM = '/q.json?b=eyJwYXJhbXMiOltdLCJtZXRob2QiOiJtZXRhIiwiaWQiOjEsImpzb25ycGMiOiIyLjAifQ==';
const PROTECTED_URLS = [
    'https://v3-cinemeta.strem.io/manifest.json',
    'http://127.0.0.1:11470/local-addon/manifest.json',
];
const OFFICIAL_URLS = [
    'https://v3-cinemeta.strem.io/manifest.json',
    'https://v3-channels.strem.io/manifest.json',
    'https://watchhub.strem.io/manifest.json',
    'https://caching.stremio.net/publicdomainmovies.now.sh/manifest.json',
    'https://opensubtitles.strem.io/stremio/v1',
    'http://127.0.0.1:11470/local-addon/manifest.json',
];

function getManifest(transportUrl) {
    if (transportUrl === 'http://127.0.0.1:11470/local-addon/manifest.json') {
        return Promise.resolve(localAddonManifest);
    }

    const legacy = transportUrl.endsWith('stremio/v1');
    return fetch(legacy ? `${transportUrl}${LEGACY_REQUEST_PARAM}` : transportUrl)
        .then((resp) => resp.json())
        .then((resp) => legacy ? legacyManifestMapper.mapManifest(resp.result) : resp);
}

function getDescriptor(transportUrl) {
    return getManifest(transportUrl)
        .then((manifest) => ({
            manifest,
            transportUrl,
            flags: {
                official: true,
                protected: PROTECTED_URLS.includes(transportUrl)
            }
        }));
}

Promise.all(OFFICIAL_URLS.map((url) => getDescriptor(url)))
    .then((descriptors) => {
        return descriptors
            .map((descriptor) => {
                const validated = validator.descriptor(descriptor);
                if (validated === null) {
                    throw new Error(`${descriptor.transportUrl} is invalid`);
                }

                return validated;
            });
    })
    .then((descriptors) => {
        fs.writeFileSync('./addons.json', JSON.stringify(descriptors, null, 4));
    });
