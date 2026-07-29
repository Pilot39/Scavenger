import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { ApiClient, ApiError, createApiClient } from '../apiClient'

/* ────────────────────────────────────────────────────────────────
   Helpers
   ──────────────────────────────────────────────────────────────── */

function mockFetch(
  response: Partial<Response> & { body?: unknown } = {}
): ReturnType<typeof vi.fn> {
  const { status = 200, statusText = 'OK', body = {}, headers: rawHeaders = {} } = response
  const headersObj = new Headers(rawHeaders as Record<string, string>)
  if (!headersObj.has('content-type')) headersObj.set('content-type', 'application/json')

  const fetchMock = vi.fn().mockResolvedValue({
    ok: status >= 200 && status < 300,
    status,
    statusText,
    headers: headersObj,
    json: vi.fn().mockResolvedValue(body),
    text: vi.fn().mockResolvedValue(JSON.stringify(body)),
  } satisfies Partial<Response>)

  vi.stubGlobal('fetch', fetchMock)
  return fetchMock
}

/* ────────────────────────────────────────────────────────────────
   Tests
   ──────────────────────────────────────────────────────────────── */

describe('ApiClient', () => {
  afterEach(() => {
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  describe('get()', () => {
    it('calls fetch with the correct URL and method', async () => {
      const fetchMock = mockFetch({ body: { items: [] } })
      const client = createApiClient({ baseUrl: 'https://api.example.com' })

      await client.get('/wastes')

      expect(fetchMock).toHaveBeenCalledOnce()
      const [url, init] = fetchMock.mock.calls[0] as [string, RequestInit]
      expect(url).toBe('https://api.example.com/wastes')
      expect(init.method).toBe('GET')
    })

    it('appends query params to the URL', async () => {
      const fetchMock = mockFetch()
      const client = createApiClient({ baseUrl: 'https://api.example.com' })

      await client.get('/wastes', { params: { page: 2, limit: 10 } })

      const [url] = fetchMock.mock.calls[0] as [string]
      expect(url).toContain('page=2')
      expect(url).toContain('limit=10')
    })

    it('returns typed data with status and durationMs', async () => {
      mockFetch({ body: { id: 1, type: 'Plastic' } })
      const client = createApiClient({ baseUrl: 'https://api.example.com' })

      const result = await client.get<{ id: number; type: string }>('/waste/1')

      expect(result.data).toEqual({ id: 1, type: 'Plastic' })
      expect(result.status).toBe(200)
      expect(typeof result.durationMs).toBe('number')
    })

    it('sets Authorization header when bearerToken is configured', async () => {
      const fetchMock = mockFetch()
      const client = createApiClient({
        baseUrl: 'https://api.example.com',
        bearerToken: 'my-token',
      })

      await client.get('/wastes')

      const [, init] = fetchMock.mock.calls[0] as [string, RequestInit]
      expect((init.headers as Record<string, string>)['Authorization']).toBe('Bearer my-token')
    })

    it('merges per-request headers', async () => {
      const fetchMock = mockFetch()
      const client = createApiClient({ baseUrl: 'https://api.example.com' })

      await client.get('/wastes', { headers: { 'X-Custom': 'yes' } })

      const [, init] = fetchMock.mock.calls[0] as [string, RequestInit]
      expect((init.headers as Record<string, string>)['X-Custom']).toBe('yes')
    })
  })

  describe('post()', () => {
    it('calls fetch with POST and serialised JSON body', async () => {
      const fetchMock = mockFetch({ status: 201, body: { id: 42 } })
      const client = createApiClient({ baseUrl: 'https://api.example.com' })

      await client.post('/wastes', { type: 'Plastic', weight: 1.5 })

      const [, init] = fetchMock.mock.calls[0] as [string, RequestInit]
      expect(init.method).toBe('POST')
      expect(init.body).toBe(JSON.stringify({ type: 'Plastic', weight: 1.5 }))
    })
  })

  describe('put()', () => {
    it('calls fetch with PUT method', async () => {
      const fetchMock = mockFetch()
      const client = createApiClient({ baseUrl: 'https://api.example.com' })

      await client.put('/waste/1', { status: 'verified' })

      const [, init] = fetchMock.mock.calls[0] as [string, RequestInit]
      expect(init.method).toBe('PUT')
    })
  })

  describe('patch()', () => {
    it('calls fetch with PATCH method', async () => {
      const fetchMock = mockFetch()
      const client = createApiClient({ baseUrl: 'https://api.example.com' })

      await client.patch('/waste/1', { weight: 2.0 })

      const [, init] = fetchMock.mock.calls[0] as [string, RequestInit]
      expect(init.method).toBe('PATCH')
    })
  })

  describe('delete()', () => {
    it('calls fetch with DELETE method and no body', async () => {
      const fetchMock = mockFetch({ status: 204 })
      const client = createApiClient({ baseUrl: 'https://api.example.com' })

      await client.delete('/waste/1')

      const [, init] = fetchMock.mock.calls[0] as [string, RequestInit]
      expect(init.method).toBe('DELETE')
      expect(init.body).toBeUndefined()
    })
  })

  describe('error handling', () => {
    it('throws ApiError with status and body for 4xx responses', async () => {
      mockFetch({ status: 404, statusText: 'Not Found', body: { error: 'waste not found' } })
      const client = createApiClient({ baseUrl: 'https://api.example.com' })

      await expect(client.get('/waste/999')).rejects.toThrow(ApiError)

      try {
        await client.get('/waste/999')
      } catch (err) {
        expect(err).toBeInstanceOf(ApiError)
        const apiErr = err as ApiError
        expect(apiErr.status).toBe(404)
        expect(apiErr.statusText).toBe('Not Found')
      }
    })

    it('throws ApiError for 5xx responses', async () => {
      mockFetch({ status: 500, statusText: 'Internal Server Error', body: {} })
      const client = createApiClient({ baseUrl: 'https://api.example.com' })

      await expect(client.get('/wastes')).rejects.toBeInstanceOf(ApiError)
    })

    it('throws ApiError with status 0 on network failure', async () => {
      vi.stubGlobal('fetch', vi.fn().mockRejectedValue(new Error('Failed to fetch')))
      const client = createApiClient({ baseUrl: 'https://api.example.com' })

      await expect(client.get('/wastes')).rejects.toThrow(ApiError)
    })

    it('throws ApiError with timeout message when AbortError is raised', async () => {
      const abortError = new Error('Aborted')
      abortError.name = 'AbortError'
      vi.stubGlobal('fetch', vi.fn().mockRejectedValue(abortError))

      const client = createApiClient({
        baseUrl: 'https://api.example.com',
        timeoutMs: 100,
      })

      await expect(client.get('/wastes')).rejects.toThrow(/timed out/)
    })
  })

  describe('setToken()', () => {
    it('updates the bearer token used for subsequent requests', async () => {
      const fetchMock = mockFetch()
      const client = new ApiClient({ baseUrl: 'https://api.example.com' })

      client.setToken('new-token')
      await client.get('/wastes')

      const [, init] = fetchMock.mock.calls[0] as [string, RequestInit]
      expect((init.headers as Record<string, string>)['Authorization']).toBe('Bearer new-token')
    })

    it('removes Authorization header when token is set to undefined', async () => {
      const fetchMock = mockFetch()
      const client = new ApiClient({
        baseUrl: 'https://api.example.com',
        bearerToken: 'old-token',
      })

      client.setToken(undefined)
      await client.get('/wastes')

      const [, init] = fetchMock.mock.calls[0] as [string, RequestInit]
      expect((init.headers as Record<string, string>)['Authorization']).toBeUndefined()
    })
  })
})
