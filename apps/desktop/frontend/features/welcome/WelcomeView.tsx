import { Icon } from '@/components/ui/Icon';
import { useWorkspaceStore } from '@/features/workspace/stores/useWorkspaceStore';
import { useWorkbenchStore } from '@/features/workbench/store/workbenchStore';
import { WorkspaceService } from '@/features/workspace/services/workspaceService';
import { useTabsStore, WELCOME_TAB_ID } from '@/features/tabs/store';
import { cn } from '@/lib/utils';

/* ------------------------------------------------------------------ */
/*  Small helper components                                            */
/* ------------------------------------------------------------------ */
function ActionButton({
  icon,
  label,
  onClick,
}: {
  icon: React.ComponentProps<typeof Icon>['name'];
  label: string;
  onClick: () => void;
}) {
  return (
    <button
      className={cn(
        'flex items-center gap-3 w-full px-4 py-2.5 rounded-lg',
        'bg-panel hover:bg-elevated-panel transition-colors',
        'text-sm text-primary font-medium',
        'border border-divider hover:border-accent',
        'focus:outline-none focus:ring-1 focus:ring-accent',
      )}
      onClick={onClick}
    >
      <Icon name={icon} size={16} className="text-accent flex-shrink-0" />
      <span>{label}</span>
    </button>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div className="space-y-3">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        {title}
      </h2>
      <div className="space-y-2">{children}</div>
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Main Welcome Screen                                                */
/* ------------------------------------------------------------------ */
export function WelcomeView() {
  const openWorkspace = useWorkspaceStore((s) => s.openWorkspaceViaDialog);
  const { activateLeftPanel, activateRightPanel } = useWorkbenchStore();

  const handleOpenFile = async () => {
    try {
      const result = await WorkspaceService.openFileDialog();
      if (result.selectedPath) {
        const { openFile } = useTabsStore.getState();
        const name = result.selectedPath.split(/[/\\]/).pop() || 'file';
        openFile(result.selectedPath, name, 'file');
        // Let the workspace store know which file is active
        useWorkspaceStore.getState().setActiveFilePath(result.selectedPath);
        await WorkspaceService.openFileInEditor(result.selectedPath);
      }
    } catch (err) {
      console.error('Failed to open file from Welcome:', err);
    }
  };

  const handleAskAi = () => {
    activateRightPanel('assistant');
  };

  const handleSettings = () => {
    activateLeftPanel('settings');
  };

  return (
    <div className="h-full w-full flex items-start justify-center overflow-y-auto bg-editor">
      <div className="max-w-xl w-full px-6 py-12 space-y-10">
        {/* Title */}
        <div className="space-y-1">
          <h1 className="text-xl font-semibold text-primary">Zaroxi Studio</h1>
          <p className="text-sm text-muted-foreground">
            Welcome &middot; start working with your workspace
          </p>
        </div>

        {/* Quick Actions */}
        <Section title="Quick Actions">
          <ActionButton
            icon="folder"
            label="Open Workspace"
            onClick={openWorkspace}
          />
          <ActionButton
            icon="file"
            label="Open File"
            onClick={handleOpenFile}
          />
          <ActionButton
            icon="assistant"
            label="AI Assistant"
            onClick={handleAskAi}
          />
          <ActionButton
            icon="settings"
            label="Settings"
            onClick={handleSettings}
          />
        </Section>

        {/* Getting Started */}
        <Section title="Getting Started">
          <ul className="text-sm text-muted-foreground space-y-2 list-outside list-disc pl-5">
            <li>
              <strong className="text-primary">Open a folder</strong> – use the
              &ldquo;Open Workspace&rdquo; button above or the folder icon in the
              activity rail.
            </li>
            <li>
              <strong className="text-primary">Browse files</strong> – once a
              workspace is opened, click any file in the Explorer panel.
            </li>
            <li>
              <strong className="text-primary">Use the command palette</strong> –{' '}
              <kbd className="px-1 py-0.5 rounded bg-muted font-mono text-xs">
                Ctrl+Shift+P
              </kbd>
            </li>
            <li>
              <strong className="text-primary">Ask the AI</strong> – toggle the
              Assistant panel with the AI icon in the activity rail.
            </li>
          </ul>
        </Section>

        {/* Keyboard Shortcuts */}
        <Section title="Keyboard Shortcuts">
          <div className="grid grid-cols-[1fr_auto] gap-x-4 gap-y-1.5 text-sm">
            <span className="text-muted-foreground">Open file</span>
            <kbd className="font-mono text-xs text-primary text-right">
              Ctrl+O
            </kbd>
            <span className="text-muted-foreground">Save file</span>
            <kbd className="font-mono text-xs text-primary text-right">
              Ctrl+S
            </kbd>
            <span className="text-muted-foreground">Command palette</span>
            <kbd className="font-mono text-xs text-primary text-right">
              Ctrl+Shift+P
            </kbd>
            <span className="text-muted-foreground">Toggle sidebar</span>
            <kbd className="font-mono text-xs text-primary text-right">
              Ctrl+B
            </kbd>
            <span className="text-muted-foreground">AI Assistant</span>
            <kbd className="font-mono text-xs text-primary text-right">
              Ctrl+Shift+A
            </kbd>
          </div>
        </Section>

        {/* Footer hint */}
        <p className="text-xs text-muted-foreground text-center pt-4 border-t border-divider">
          You can close this tab with the <code className="font-mono">×</code>{' '}
          button. It will re‑open automatically when all other tabs are closed.
        </p>
      </div>
    </div>
  );
}
