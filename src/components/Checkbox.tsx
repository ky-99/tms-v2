import { Checkbox as KobalteCheckbox } from "@kobalte/core/checkbox";
import { JSX, splitProps } from "solid-js";
import { cn } from "../lib/utils";

interface CheckboxProps {
  checked?: boolean;
  onChange?: (checked: boolean) => void;
  label?: string;
  class?: string;
}

export function Checkbox(props: CheckboxProps) {
  const [local, others] = splitProps(props, ["class", "label", "checked", "onChange"]);

  return (
    <KobalteCheckbox
      checked={local.checked}
      onChange={local.onChange}
      class={cn("flex items-center space-x-2", local.class)}
      {...others}
    >
      <KobalteCheckbox.Input class="peer" />
      <KobalteCheckbox.Control
        class={cn(
          "peer h-4 w-4 shrink-0 rounded-sm border border-primary ring-offset-background",
          "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
          "disabled:cursor-not-allowed disabled:opacity-50",
          "data-[checked]:bg-primary data-[checked]:text-primary-foreground"
        )}
      >
        <KobalteCheckbox.Indicator class="flex items-center justify-center text-current">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="h-4 w-4"
          >
            <polyline points="20 6 9 17 4 12" />
          </svg>
        </KobalteCheckbox.Indicator>
      </KobalteCheckbox.Control>
      {local.label && (
        <KobalteCheckbox.Label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
          {local.label}
        </KobalteCheckbox.Label>
      )}
    </KobalteCheckbox>
  );
}
