import { Dialog as KobalteDialog } from "@kobalte/core/dialog";
import { JSX } from "solid-js";
import { cn } from "../lib/utils";

interface DialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title: string;
  children: JSX.Element;
}

export function Dialog(props: DialogProps) {
  return (
    <KobalteDialog open={props.open} onOpenChange={props.onOpenChange}>
      <KobalteDialog.Portal>
        <KobalteDialog.Overlay class="fixed inset-0 z-50 bg-black/80 data-[expanded]:animate-in data-[closed]:animate-out data-[closed]:fade-out-0 data-[expanded]:fade-in-0" />
        <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
          <KobalteDialog.Content class={cn(
            "relative z-50 w-full max-w-lg bg-background rounded-lg border border-border shadow-lg",
            "data-[expanded]:animate-in data-[closed]:animate-out data-[closed]:fade-out-0 data-[expanded]:fade-in-0 data-[closed]:zoom-out-95 data-[expanded]:zoom-in-95"
          )}>
            <div class="flex flex-col space-y-1.5 p-6 pb-0">
              <div class="flex items-center justify-between">
                <KobalteDialog.Title class="text-lg font-semibold leading-none tracking-tight">
                  {props.title}
                </KobalteDialog.Title>
                <KobalteDialog.CloseButton class={cn(
                  "rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100",
                  "focus:outline-none focus:ring-1 focus:ring-ring focus:ring-offset-2",
                  "disabled:pointer-events-none"
                )}>
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-4 w-4"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M6 18L18 6M6 6l12 12"
                    />
                  </svg>
                  <span class="sr-only">Close</span>
                </KobalteDialog.CloseButton>
              </div>
            </div>
            <KobalteDialog.Description class="sr-only">
              {props.title}
            </KobalteDialog.Description>
            <div class="p-6">
              {props.children}
            </div>
          </KobalteDialog.Content>
        </div>
      </KobalteDialog.Portal>
    </KobalteDialog>
  );
}
