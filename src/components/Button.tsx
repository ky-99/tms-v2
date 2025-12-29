import { JSX, splitProps } from "solid-js";
import { cn } from "../lib/utils";

type ButtonVariant = "default" | "secondary" | "destructive" | "outline" | "ghost";

interface ButtonProps extends JSX.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
}

const variantClasses: Record<ButtonVariant, string> = {
  default: "bg-primary text-primary-foreground hover:bg-primary/90",
  secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
  destructive: "bg-destructive text-destructive-foreground hover:bg-destructive/90",
  outline: "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
  ghost: "hover:bg-accent hover:text-accent-foreground",
};

export function Button(props: ButtonProps) {
  const [local, others] = splitProps(props, ["variant", "class", "children"]);
  const variant = local.variant || "default";

  return (
    <button
      class={cn(
        "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 px-4 py-2",
        variantClasses[variant],
        local.class
      )}
      {...others}
    >
      {local.children}
    </button>
  );
}
