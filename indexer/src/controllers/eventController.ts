import http from 'http';
import { getEvents } from '../services/eventService';

export async function handleEventQuery(
  _req: http.IncomingMessage,
  res: http.ServerResponse,
  url: URL
): Promise<void> {
  const result = await getEvents({
    eventType: url.searchParams.get('type'),
    fromLedger: url.searchParams.get('from'),
    toLedger: url.searchParams.get('to'),
    limit: url.searchParams.get('limit'),
    offset: url.searchParams.get('offset'),
    contractId: url.searchParams.get('contractId'),
    txHash: url.searchParams.get('txHash'),
  });

  res.writeHead(200);
  res.end(JSON.stringify(result));
}

export function handleEventStream(
  req: http.IncomingMessage,
  res: http.ServerResponse,
  sseClients: Set<http.ServerResponse>
): void {
  res.writeHead(200, {
    'Content-Type': 'text/event-stream',
    'Cache-Control': 'no-cache',
    Connection: 'keep-alive',
    'X-Accel-Buffering': 'no',
  });

  res.write('data: {"type":"connected"}\n\n');
  sseClients.add(res);

  const keepAlive = setInterval(() => {
    res.write(':keepalive\n\n');
  }, 15000);

  req.on('close', () => {
    sseClients.delete(res);
    clearInterval(keepAlive);
  });
}
