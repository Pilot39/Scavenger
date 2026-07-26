import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';

interface BarChartProps {
  data: Record<string, unknown>[];
  xKey: string;
  bars: { key: string; color: string; name?: string }[];
}

export function BarChartComponent({ data, xKey, bars }: BarChartProps) {
  return (
    <ResponsiveContainer width="100%" height="100%">
      <BarChart data={data} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
        <CartesianGrid strokeDasharray="3 3" />
        <XAxis dataKey={xKey} />
        <YAxis />
        <Tooltip />
        <Legend />
        {bars.map((bar) => (
          <Bar key={bar.key} dataKey={bar.key} fill={bar.color} name={bar.name || bar.key} />
        ))}
      </BarChart>
    </ResponsiveContainer>
  );
}
