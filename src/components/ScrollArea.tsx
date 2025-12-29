import { JSX, splitProps } from "solid-js";
import { cn } from "../lib/utils";

interface ScrollAreaProps extends JSX.HTMLAttributes<HTMLDivElement> {}

export function ScrollArea(props: ScrollAreaProps) {
  const [local, others] = splitProps(props, ["class", "children"]);

  return (
    <div
      class={cn("relative overflow-hidden", local.class)}
      {...others}
    >
      <div class="h-full w-full overflow-auto">
        {local.children}
      </div>
    </div>
  );
}
