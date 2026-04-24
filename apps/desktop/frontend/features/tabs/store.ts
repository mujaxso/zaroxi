import { create } from 'zustand';
import { devtools } from 'zustand/middleware';

export type TabKind = 'file' | 'welcome';

export interface Tab {
  id: string;
  title: string;
  isDirty: boolean;
  kind: TabKind;
}

/** The reserved id for the built‑in Welcome tab. */
export const WELCOME_TAB_ID = '__welcome__';

interface TabsState {
  tabs: Tab[];
  activeTabId: string | null;
  openFile: (id: string, title: string, kind?: TabKind) => void;
  closeTab: (id: string) => void;
  setActiveTab: (id: string) => void;
  markDirty: (id: string) => void;
  markClean: (id: string) => void;
}

export const useTabsStore = create<TabsState>()(
  devtools(
    (set, get) => ({
      tabs: [],
      activeTabId: null,

      openFile: (id, title, kind = 'file') => {
        const { tabs } = get();
        const existing = tabs.find((t) => t.id === id);
        if (existing) {
          set({ activeTabId: id });
          return;
        }
        const newTab: Tab = {
          id,
          title,
          isDirty: kind === 'file' ? false : false, // special tabs are never dirty
          kind,
        };
        set({
          tabs: [...tabs, newTab],
          activeTabId: id,
        });
      },

      closeTab: (id) => {
        const { tabs, activeTabId } = get();
        const tab = tabs.find((t) => t.id === id);
        if (!tab) return;
        // completely block closing dirty tabs (no prompts, no close)
        if (tab.isDirty) {
          return;
        }
        const idx = tabs.findIndex((t) => t.id === id);
        if (idx === -1) return;

        const newTabs = tabs.filter((t) => t.id !== id);
        let newActive = activeTabId;
        if (activeTabId === id) {
          if (idx < newTabs.length) {
            newActive = newTabs[idx].id;
          } else if (newTabs.length > 0) {
            newActive = newTabs[newTabs.length - 1].id;
          } else {
            newActive = null;
          }
        }
        set({ tabs: newTabs, activeTabId: newActive });

        // If the last tab was closed, re‑open the Welcome tab automatically.
        if (newTabs.length === 0) {
          get().openFile(WELCOME_TAB_ID, 'Welcome', 'welcome');
        }
      },

      setActiveTab: (id) => {
        set({ activeTabId: id });
      },

      markDirty: (id) => {
        set((state) => ({
          tabs: state.tabs.map((t) =>
            t.id === id ? { ...t, isDirty: true } : t
          ),
        }));
      },

      markClean: (id) => {
        set((state) => ({
          tabs: state.tabs.map((t) =>
            t.id === id ? { ...t, isDirty: false } : t
          ),
        }));
      },
    }),
    { name: 'tabs-store' }
  )
);
