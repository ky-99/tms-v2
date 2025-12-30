import { A, useLocation } from "@solidjs/router";
import { Show } from "solid-js";

// Simple SVG icons matching Lucide style
function ListTodoIcon() {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <rect x="3" y="5" width="6" height="6" rx="1" />
      <path d="m3 17 2 2 4-4" />
      <path d="M13 6h8" />
      <path d="M13 12h8" />
      <path d="M13 18h8" />
    </svg>
  );
}

function CheckCircle2Icon() {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <circle cx="12" cy="12" r="10" />
      <path d="m9 12 2 2 4-4" />
    </svg>
  );
}

function ArchiveIcon() {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <rect x="2" y="4" width="20" height="5" rx="1" />
      <path d="M4 9v9a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9" />
      <path d="M10 13h4" />
    </svg>
  );
}

function TagIcon() {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M12 2H2v10l9.29 9.29c.94.94 2.48.94 3.42 0l6.58-6.58c.94-.94.94-2.48 0-3.42L12 2Z" />
      <path d="M7 7h.01" />
    </svg>
  );
}

export function Header() {
  const location = useLocation();
  const pathname = () => location.pathname;

  return (
    <header class="border-b border-border bg-background">
      <nav class="flex h-12 items-center px-4 gap-2 relative">
        <A href="/" class="relative z-10">
          <div
            class={`flex items-center gap-2 px-4 py-2 rounded-lg transition-all duration-300 ${
              pathname() === "/"
                ? "text-foreground bg-primary/10"
                : "text-muted-foreground hover:text-foreground"
            }`}
          >
            <ListTodoIcon />
            <span class="text-sm font-medium">Tasks</span>
          </div>
        </A>
        <A href="/tags" class="relative z-10">
          <div
            class={`flex items-center gap-2 px-4 py-2 rounded-lg transition-all duration-300 ${
              pathname() === "/tags"
                ? "text-foreground bg-primary/10"
                : "text-muted-foreground hover:text-foreground"
            }`}
          >
            <TagIcon />
            <span class="text-sm font-medium">Tags</span>
          </div>
        </A>
        <A href="/completed" class="relative z-10">
          <div
            class={`flex items-center gap-2 px-4 py-2 rounded-lg transition-all duration-300 ${
              pathname() === "/completed"
                ? "text-foreground bg-primary/10"
                : "text-muted-foreground hover:text-foreground"
            }`}
          >
            <CheckCircle2Icon />
            <span class="text-sm font-medium">Completed</span>
          </div>
        </A>
        <A href="/archive" class="relative z-10">
          <div
            class={`flex items-center gap-2 px-4 py-2 rounded-lg transition-all duration-300 ${
              pathname() === "/archive"
                ? "text-foreground bg-primary/10"
                : "text-muted-foreground hover:text-foreground"
            }`}
          >
            <ArchiveIcon />
            <span class="text-sm font-medium">Archive</span>
          </div>
        </A>
        {/* Drag region for empty header space */}
        <div class="flex-1 self-stretch" data-tauri-drag-region />
      </nav>
    </header>
  );
}
