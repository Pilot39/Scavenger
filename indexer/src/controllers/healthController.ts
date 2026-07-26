import http from 'http';

export function handleHealth(_req: http.IncomingMessage, res: http.ServerResponse): void {
  res.writeHead(200);
  res.end(JSON.stringify({ status: 'ok', timestamp: new Date().toISOString() }));
}

export function handleMetrics(
  _req: http.IncomingMessage,
  res: http.ServerResponse,
  metrics: Record<string, unknown>
): void {
  res.writeHead(200);
  res.end(JSON.stringify(metrics));
}
