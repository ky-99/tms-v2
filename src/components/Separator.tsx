import { Separator as KobalteSeparator } from "@kobalte/core/separator";
import { JSX, splitProps } from "solid-js";
import { cn } from "../lib/utils";

interface SeparatorProps extends JSX.HTMLAttributes<HTMLHRElement> {
  orientation?: "horizontal" | "vertical";
}

export function Separator(props: SeparatorProps) {
  const [local, others] = splitProps(props, ["class", "orientation"]);
  const orientation = local.orientation || "horizontal";

  return (
    <KobalteSeparator
      orientation={orientation}
      class={cn(
        "shrink-0 bg-border",
        orientation === "horizontal" ? "h-[1px] w-full" : "h-full w-[1px]",
        local.class
      )}
      {...others}
    />
  );
}
