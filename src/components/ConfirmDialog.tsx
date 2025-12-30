import { Dialog as KobalteDialog } from "@kobalte/core/dialog";
import { createSignal, Show, JSX } from "solid-js";
import { cn } from "../lib/utils";
import { Button } from "./Button";
import { Input } from "./Input";

interface ConfirmDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title: string;
  description: string;
  confirmText?: string;
  cancelText?: string;
  variant?: "default" | "destructive";
  onConfirm: () => void | Promise<void>;
  requireVerification?: boolean;
  verificationText?: string;
  verificationLabel?: string;
  verificationPlaceholder?: string;
}

/**
 * 確認ダイアログコンポーネント
 *
 * - 汎用的な確認ダイアログ
 * - 破壊的操作にはテキスト入力による確認を要求可能
 * - Kobalte Dialogベース
 */
export function ConfirmDialog(props: ConfirmDialogProps) {
  const [verificationInput, setVerificationInput] = createSignal("");
  const [isSubmitting, setIsSubmitting] = createSignal(false);

  const confirmText = () => props.confirmText || "Confirm";
  const cancelText = () => props.cancelText || "Cancel";
  const variant = () => props.variant || "default";

  const isVerificationValid = () => {
    if (!props.requireVerification) return true;
    return verificationInput().trim() === props.verificationText?.trim();
  };

  const handleConfirm = async () => {
    if (!isVerificationValid()) return;

    setIsSubmitting(true);
    try {
      await props.onConfirm();
      handleClose();
    } catch (error) {
      console.error("Confirm action failed:", error);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleClose = () => {
    setVerificationInput("");
    props.onOpenChange(false);
  };

  return (
    <KobalteDialog open={props.open} onOpenChange={(open) => {
      if (!open) handleClose();
    }}>
      <KobalteDialog.Portal>
        <KobalteDialog.Overlay class="fixed inset-0 z-50 bg-black/80 data-[expanded]:animate-in data-[closed]:animate-out data-[closed]:fade-out-0 data-[expanded]:fade-in-0 overflow-hidden" style={{ "border-radius": "8px" }} />
        <div class="fixed inset-0 z-50 flex items-center justify-center p-4 overflow-hidden" style={{ "border-radius": "8px" }}>
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
                  "focus:outline-none",
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
              {props.description}
            </KobalteDialog.Description>
            <div class="p-6 space-y-4">
              <p class="text-sm text-muted-foreground">
                {props.description}
              </p>

              <Show when={props.requireVerification}>
                <div class="space-y-2">
                  <label class="text-sm font-medium">
                    {props.verificationLabel || "Type to confirm:"}
                  </label>
                  <Input
                    type="text"
                    placeholder={props.verificationPlaceholder || "Enter text to confirm"}
                    value={verificationInput()}
                    onInput={(e) => setVerificationInput(e.currentTarget.value)}
                    class={cn(
                      "border-border",
                      !isVerificationValid() && verificationInput().length > 0 && "border-destructive"
                    )}
                  />
                  <div class="h-5">
                    <Show when={!isVerificationValid() && verificationInput().length > 0}>
                      <p class="text-xs text-destructive">
                        Text does not match. Please type "{props.verificationText}" to confirm.
                      </p>
                    </Show>
                  </div>
                </div>
              </Show>

              <div class="flex gap-2 justify-end pt-2">
                <Button
                  type="button"
                  variant="secondary"
                  onClick={handleClose}
                  disabled={isSubmitting()}
                >
                  {cancelText()}
                </Button>
                <Button
                  type="button"
                  variant={variant()}
                  onClick={handleConfirm}
                  disabled={!isVerificationValid() || isSubmitting()}
                >
                  {isSubmitting() ? "Processing..." : confirmText()}
                </Button>
              </div>
            </div>
          </KobalteDialog.Content>
        </div>
      </KobalteDialog.Portal>
    </KobalteDialog>
  );
}
