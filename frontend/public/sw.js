// Service Worker: 静的アセットのみをキャッシュ対象とし、API通信には一切関与しない。
// Viteがビルド時に付与するコンテンツハッシュ付きファイル名（/assets/配下）は
// 内容が変わればファイル名も変わるため、cache-firstで長期キャッシュしても安全。
// HTMLナビゲーションはnetwork-firstとし、オフライン時のみキャッシュへフォールバックする。

const STATIC_CACHE = 'hoshika-static-v1'
const RUNTIME_CACHE = 'hoshika-runtime-v1'
const CURRENT_CACHES = [STATIC_CACHE, RUNTIME_CACHE]

self.addEventListener('install', () => {
  self.skipWaiting()
})

self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches
      .keys()
      .then((keys) =>
        Promise.all(
          keys
            .filter((key) => !CURRENT_CACHES.includes(key))
            .map((key) => caches.delete(key)),
        ),
      )
      .then(() => self.clients.claim()),
  )
})

self.addEventListener('fetch', (event) => {
  const { request } = event

  if (request.method !== 'GET') return

  const url = new URL(request.url)
  if (url.origin !== self.location.origin || url.pathname.startsWith('/api/')) {
    return
  }

  if (url.pathname.startsWith('/assets/')) {
    event.respondWith(
      caches.match(request).then(
        (cached) =>
          cached ||
          fetch(request).then((response) => {
            const clone = response.clone()
            caches.open(STATIC_CACHE).then((cache) => cache.put(request, clone))
            return response
          }),
      ),
    )
    return
  }

  if (request.mode === 'navigate') {
    event.respondWith(
      fetch(request)
        .then((response) => {
          const clone = response.clone()
          caches.open(RUNTIME_CACHE).then((cache) => cache.put(request, clone))
          return response
        })
        .catch(() => caches.match(request).then((cached) => cached || caches.match('/'))),
    )
  }
})
