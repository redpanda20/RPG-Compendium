var cacheName = 'rpg-compendium-pwa';
var filesToCache = [
  './',
  './index.html',
  './web-app.js',
  './web-app_bg.wasm',
  './manifest.json',
  './favicon.png',
  './icons-vector.svg',
  './icons-512.png',
  './icons-192.png',
];

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

/* Serve cached content when offline */
self.addEventListener('fetch', function (e) {
  e.respondWith(
    caches.match(e.request).then(function (response) {
      return response || fetch(e.request);
    })
  );
});