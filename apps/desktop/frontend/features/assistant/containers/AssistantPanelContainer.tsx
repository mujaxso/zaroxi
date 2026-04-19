export function AssistantPanelContainer() {
  return (
    <div className="h-full flex flex-col">
      <div className="border-b border-border px-4 py-3">
        <h2 className="font-semibold">AI Assistant</h2>
        <p className="text-sm text-muted-foreground">Ask questions about your code</p>
      </div>
      <div className="flex-1 p-4">
        <div className="space-y-4">
          <div className="p-3 bg-muted rounded-lg">
            <p className="text-sm">Welcome! I'm your AI assistant. How can I help you today?</p>
          </div>
          <div className="p-3 bg-sidebar border border-border rounded-lg">
            <p className="text-sm">Try asking me to explain code, refactor functions, or generate tests.</p>
          </div>
        </div>
      </div>
      <div className="border-t border-border p-4">
        <div className="flex space-x-2">
          <input
            type="text"
            placeholder="Ask a question..."
            className="flex-1 px-3 py-2 text-sm border border-input rounded bg-background focus:outline-none focus:ring-2 focus:ring-primary"
          />
          <button className="px-4 py-2 text-sm bg-primary text-primary-foreground rounded hover:bg-primary/90">
            Send
          </button>
        </div>
      </div>
    </div>
  );
}
