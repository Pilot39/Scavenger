import http from 'http';
import { startReplay } from '../services/replayService';

export async function handleReplay(req: http.IncomingMessage, res: http.ServerResponse): Promise<void> {
  if (req.method !== 'POST') {
    res.writeHead(405);
    res.end(JSON.stringify({ error: 'Method not allowed' }));
    return;
  }

  let body = '';
  for await (const chunk of req) body += chunk;

  try {
    const { fromLedger, toLedger, eventTypes } = JSON.parse(body);
    if (typeof fromLedger !== 'number') {
      res.writeHead(400);
      res.end(JSON.stringify({ error: 'fromLedger is required' }));
      return;
    }

    const result = await startReplay({ fromLedger, toLedger, eventTypes });

    res.writeHead(202);
    res.end(JSON.stringify(result));
  } catch (err) {
    res.writeHead(400);
    res.end(JSON.stringify({ error: String(err) }));
  }
}

export async function handleReplayStatus(_req: http.IncomingMessage, res: http.ServerResponse): Promise<void> {
  res.writeHead(200);
  res.end(JSON.stringify({ status: 'available', message: 'Replay status tracking is active' }));
}
