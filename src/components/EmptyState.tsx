import { JSX, splitProps } from "solid-js";
import { cn } from "../lib/utils";

interface EmptyStateProps extends JSX.HTMLAttributes<HTMLDivElement> {
  icon?: JSX.Element;
  title?: string;
  description?: string;
  action?: JSX.Element;
}

export function EmptyState(props: EmptyStateProps) {
  const [local, others] = splitProps(props, ["class", "icon", "title", "description", "action", "children"]);

  return (
    <div
      class={cn(
        "flex min-h-[400px] flex-col items-center justify-center rounded-lg border border-dashed border-border bg-background p-8 text-center",
        local.class
      )}
      {...others}
    >
      {local.icon && (
        <div class="mb-4 flex h-20 w-20 items-center justify-center rounded-full bg-muted">
          {local.icon}
        </div>
      )}
      {local.title && (
        <h3 class="mb-2 text-lg font-semibold">{local.title}</h3>
      )}
      {local.description && (
        <p class="mb-4 text-sm text-muted-foreground max-w-sm">
          {local.description}
        </p>
      )}
      {local.action && (
        <div class="mt-4">
          {local.action}
        </div>
      )}
      {local.children}
    </div>
  );
}
