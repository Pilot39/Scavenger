import create from 'zustand';

interface Participant {
  address: string;
  name: string;
  role: string;
}

interface Stats {
  total_waste: number;
  total_rewards: number;
}

interface AppStore {
  participant: Participant | null;
  stats: Stats | null;
  setParticipant: (participant: Participant) => void;
  setStats: (stats: Stats) => void;
}

export const useAppStore = create<AppStore>((set) => ({
  participant: null,
  stats: null,
  setParticipant: (participant) => set({ participant }),
  setStats: (stats) => set({ stats }),
}));
