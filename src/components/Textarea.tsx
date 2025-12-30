import { JSX, splitProps } from "solid-js";
import { cn } from "../lib/utils";

interface TextareaProps extends JSX.TextareaHTMLAttributes<HTMLTextAreaElement> {
  label?: string;
  error?: string;
  ref?: HTMLTextAreaElement | ((el: HTMLTextAreaElement) => void);
}

export function Textarea(props: TextareaProps) {
  const [local, others] = splitProps(props, ["label", "error", "class", "ref"]);

  return (
    <div class="flex flex-col gap-1.5">
      {local.label && (
        <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
          {local.label}
        </label>
      )}
      <textarea
        ref={local.ref}
        class={cn(
          "flex min-h-20 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring/30 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
          local.error && "border-destructive focus-visible:ring-destructive/30",
          local.class
        )}
        {...others}
      />
      {local.error && (
        <span class="text-sm text-destructive">{local.error}</span>
      )}
    </div>
  );
}
