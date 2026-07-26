import http from 'http';
import { getRecentAlerts } from '../services/alertService';

export async function handleAlertQuery(
  _req: http.IncomingMessage,
  res: http.ServerResponse,
  url: URL
): Promise<void> {
  const alerts = await getRecentAlerts(url.searchParams.get('limit'));
  res.writeHead(200);
  res.end(JSON.stringify({ alerts }));
}
