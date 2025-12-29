import { JSX, splitProps } from "solid-js";
import { cn } from "../lib/utils";

interface InputProps extends JSX.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
}

export function Input(props: InputProps) {
  const [local, others] = splitProps(props, ["label", "error", "class"]);

  return (
    <div class="flex flex-col gap-1.5">
      {local.label && (
        <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
          {local.label}
        </label>
      )}
      <input
        class={cn(
          "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring/30 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
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
