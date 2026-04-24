import { useTabsStore } from './store';
import { TabItem } from './TabItem';

export function TabStrip() {
  const { tabs, activeTabId } = useTabsStore();

  if (tabs.length === 0) {
    return null;
  }

  return (
    <div
      className="flex items-start h-9 overflow-x-auto overflow-y-hidden bg-activity-rail text-activity-rail-foreground"
      style={{ scrollbarWidth: 'none', msOverflowStyle: 'none', borderBottom: '0.5px solid var(--color-divider-subtle)' }}
      data-no-drag="true"
    >
      {tabs.map((tab) => (
        <TabItem key={tab.id} tab={tab} isActive={tab.id === activeTabId} />
      ))}
      {/** small right‑side spacer to give a bit of room after the last tab */}
      <div className="flex-shrink-0 w-4" />
    </div>
  );
}
