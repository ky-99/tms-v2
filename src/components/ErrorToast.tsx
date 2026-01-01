import { For, JSX, Show } from "solid-js";
import { toastStore, type ToastCategory } from "../stores/toastStore";
import { NetworkErrorIcon, ValidationErrorIcon, ServerErrorIcon, XIcon } from "./icons";

export function ErrorToastProvider(props: { children: JSX.Element }) {
  const getIcon = (category: ToastCategory) => {
    switch (category) {
      case "network":
        return <NetworkErrorIcon />;
      case "validation":
        return <ValidationErrorIcon />;
      case "server":
        return <ServerErrorIcon />;
      default:
        return null;
    }
  };

  return (
    <>
      {props.children}

      {/* Toast Container */}
      <div class="fixed bottom-4 right-4 z-[9999] flex flex-col gap-2 w-80 pointer-events-none">
        <For each={toastStore.toasts()}>
          {(toast) => (
            <div
              class="flex items-center gap-3 rounded-lg border border-destructive bg-destructive/10 px-4 py-3 shadow-lg backdrop-blur-sm animate-in slide-in-from-right pointer-events-auto"
              role="alert"
            >
              <div class="text-destructive flex-shrink-0">
                {getIcon(toast.category)}
              </div>

              <div class="flex-1 text-sm text-foreground">
                {toast.message}
              </div>

              <button
                class="text-muted-foreground hover:text-foreground flex-shrink-0 transition-colors"
                onClick={() => toastStore.dismiss(toast.id)}
                aria-label="Close"
              >
                <XIcon />
              </button>
            </div>
          )}
        </For>
      </div>
    </>
  );
}
