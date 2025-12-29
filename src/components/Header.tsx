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
      <nav class="flex h-12 items-end px-4">
        <A href="/" class="group">
          <div
            class={`flex items-center gap-2 px-4 pb-2 pt-3 rounded-t-lg transition-colors relative ${
              pathname() === "/"
                ? "bg-card text-primary border-t-2 border-t-primary"
                : "bg-background/50 text-muted-foreground hover:bg-card/50 hover:text-foreground"
            }`}
          >
            <ListTodoIcon />
            <span class="text-sm font-medium">Tasks</span>
            <Show when={pathname() === "/"}>
              <div class="absolute bottom-0 left-0 right-0 h-[2px] bg-card" />
            </Show>
          </div>
        </A>
        <A href="/completed" class="group -ml-2">
          <div
            class={`flex items-center gap-2 px-4 pb-2 pt-3 rounded-t-lg transition-colors relative ${
              pathname() === "/completed"
                ? "bg-card text-primary border-t-2 border-t-primary"
                : "bg-background/50 text-muted-foreground hover:bg-card/50 hover:text-foreground"
            }`}
          >
            <CheckCircle2Icon />
            <span class="text-sm font-medium">Completed</span>
            <Show when={pathname() === "/completed"}>
              <div class="absolute bottom-0 left-0 right-0 h-[2px] bg-card" />
            </Show>
          </div>
        </A>
        <A href="/archive" class="group -ml-2">
          <div
            class={`flex items-center gap-2 px-4 pb-2 pt-3 rounded-t-lg transition-colors relative ${
              pathname() === "/archive"
                ? "bg-card text-primary border-t-2 border-t-primary"
                : "bg-background/50 text-muted-foreground hover:bg-card/50 hover:text-foreground"
            }`}
          >
            <ArchiveIcon />
            <span class="text-sm font-medium">Archive</span>
            <Show when={pathname() === "/archive"}>
              <div class="absolute bottom-0 left-0 right-0 h-[2px] bg-card" />
            </Show>
          </div>
        </A>
        <A href="/tags" class="group -ml-2">
          <div
            class={`flex items-center gap-2 px-4 pb-2 pt-3 rounded-t-lg transition-colors relative ${
              pathname() === "/tags"
                ? "bg-card text-primary border-t-2 border-t-primary"
                : "bg-background/50 text-muted-foreground hover:bg-card/50 hover:text-foreground"
            }`}
          >
            <TagIcon />
            <span class="text-sm font-medium">Tags</span>
            <Show when={pathname() === "/tags"}>
              <div class="absolute bottom-0 left-0 right-0 h-[2px] bg-card" />
            </Show>
          </div>
        </A>
      </nav>
    </header>
  );
}
