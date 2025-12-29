import { createSignal, For, Show, createEffect } from "solid-js";
import { Popover as KobaltePopover } from "@kobalte/core/popover";
import { cn } from "../lib/utils";
import { Button } from "./Button";
import { Input } from "./Input";
import type { Tag } from "../types/tag";
import { PRESET_TAG_COLORS } from "../types/tag";

interface TagInputProps {
  selectedTags: string[]; // Array of tag names
  onTagsChange: (tags: string[]) => void;
  availableTags: Tag[];
  onCreateTag?: (name: string, color: string) => Promise<Tag>;
  placeholder?: string;
}

// Icon components
function XIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path d="M18 6 6 18M6 6l12 12" />
    </svg>
  );
}

function PlusIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path d="M5 12h14M12 5v14" />
    </svg>
  );
}

/**
 * タグ入力コンポーネント
 *
 * - チップ入力スタイルで既存タグを表示・削除
 * - オートコンプリートで既存タグを検索・追加
 * - インライン新規タグ作成（名前+プリセット8色から選択）
 */
export function TagInput(props: TagInputProps) {
  const [inputValue, setInputValue] = createSignal("");
  const [isAutocompleteOpen, setIsAutocompleteOpen] = createSignal(false);
  const [isCreateMode, setIsCreateMode] = createSignal(false);
  const [selectedColor, setSelectedColor] = createSignal(PRESET_TAG_COLORS[0].value);
  const [isCreating, setIsCreating] = createSignal(false);

  // Filter available tags based on input and exclude already selected tags
  const filteredSuggestions = () => {
    const input = inputValue().toLowerCase().trim();
    if (!input) return [];

    return props.availableTags.filter(
      (tag) =>
        tag.name.toLowerCase().includes(input) &&
        !props.selectedTags.includes(tag.name)
    );
  };

  // Check if input matches an existing tag exactly
  const exactMatch = () => {
    const input = inputValue().trim();
    return props.availableTags.find(
      (tag) => tag.name.toLowerCase() === input.toLowerCase()
    );
  };

  // Check if we should show "Create new tag" option
  const shouldShowCreateOption = () => {
    const input = inputValue().trim();
    return (
      input.length > 0 &&
      !exactMatch() &&
      !props.selectedTags.some((t) => t.toLowerCase() === input.toLowerCase())
    );
  };

  // Add tag by name
  const addTag = (tagName: string) => {
    if (!props.selectedTags.includes(tagName)) {
      props.onTagsChange([...props.selectedTags, tagName]);
    }
    setInputValue("");
    setIsAutocompleteOpen(false);
    setIsCreateMode(false);
  };

  // Remove tag by name
  const removeTag = (tagName: string) => {
    props.onTagsChange(props.selectedTags.filter((t) => t !== tagName));
  };

  // Handle input change
  const handleInputChange = (value: string) => {
    setInputValue(value);
    setIsAutocompleteOpen(value.length > 0);
    setIsCreateMode(false);
  };

  // Handle Enter key
  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === "Enter") {
      e.preventDefault();
      const match = exactMatch();
      if (match) {
        addTag(match.name);
      } else if (shouldShowCreateOption()) {
        setIsCreateMode(true);
      }
    } else if (e.key === "Escape") {
      setInputValue("");
      setIsAutocompleteOpen(false);
      setIsCreateMode(false);
    }
  };

  // Create new tag
  const handleCreateTag = async () => {
    const tagName = inputValue().trim();
    if (!tagName || !props.onCreateTag) return;

    setIsCreating(true);
    try {
      await props.onCreateTag(tagName, selectedColor());
      addTag(tagName);
      setSelectedColor(PRESET_TAG_COLORS[0].value); // Reset to first color
    } catch (error) {
      console.error("Failed to create tag:", error);
    } finally {
      setIsCreating(false);
      setIsCreateMode(false);
    }
  };

  // Get tag color from available tags
  const getTagColor = (tagName: string): string | undefined => {
    return props.availableTags.find((t) => t.name === tagName)?.color;
  };

  return (
    <div class="space-y-2">
      {/* Selected tags (chips) */}
      <Show when={props.selectedTags.length > 0}>
        <div class="flex flex-wrap gap-2">
          <For each={props.selectedTags}>
            {(tagName) => {
              const color = getTagColor(tagName);
              return (
                <div
                  class="flex items-center gap-1 rounded-md px-2 py-1 text-xs font-medium"
                  style={{
                    "background-color": color ? `${color}20` : "#e5e7eb",
                    color: color || "#374151",
                  }}
                >
                  <span>{tagName}</span>
                  <button
                    type="button"
                    onClick={() => removeTag(tagName)}
                    class="hover:opacity-70 transition-opacity"
                    aria-label={`Remove ${tagName}`}
                  >
                    <XIcon />
                  </button>
                </div>
              );
            }}
          </For>
        </div>
      </Show>

      {/* Input field with autocomplete */}
      <div class="relative">
        <Input
          type="text"
          placeholder={props.placeholder || "Add tags..."}
          value={inputValue()}
          onInput={(e) => handleInputChange(e.currentTarget.value)}
          onKeyDown={handleKeyDown}
          class="w-full"
        />

        {/* Autocomplete dropdown */}
        <Show when={isAutocompleteOpen() && (filteredSuggestions().length > 0 || shouldShowCreateOption())}>
          <div class="absolute z-50 mt-1 w-full rounded-md border border-border bg-card shadow-lg">
            <div class="max-h-60 overflow-y-auto p-1">
              {/* Existing tag suggestions */}
              <For each={filteredSuggestions()}>
                {(tag) => (
                  <button
                    type="button"
                    onClick={() => addTag(tag.name)}
                    class="flex w-full items-center gap-2 rounded-sm px-3 py-2 text-sm hover:bg-secondary transition-colors text-left"
                  >
                    <Show when={tag.color}>
                      <div
                        class="h-3 w-3 rounded-full flex-shrink-0"
                        style={{ "background-color": tag.color }}
                      />
                    </Show>
                    <span>{tag.name}</span>
                    <span class="ml-auto text-xs text-muted-foreground">({tag.usageCount})</span>
                  </button>
                )}
              </For>

              {/* Create new tag option */}
              <Show when={shouldShowCreateOption()}>
                <button
                  type="button"
                  onClick={() => setIsCreateMode(true)}
                  class="flex w-full items-center gap-2 rounded-sm px-3 py-2 text-sm hover:bg-secondary transition-colors text-left border-t border-border"
                >
                  <PlusIcon />
                  <span>Create "{inputValue().trim()}"</span>
                </button>
              </Show>
            </div>
          </div>
        </Show>
      </div>

      {/* Create tag dialog (inline) */}
      <Show when={isCreateMode()}>
        <div class="rounded-md border border-border bg-card p-4 space-y-3">
          <div>
            <p class="text-sm font-medium mb-2">Create new tag: "{inputValue().trim()}"</p>
            <p class="text-xs text-muted-foreground mb-3">Select a color:</p>
            <div class="grid grid-cols-8 gap-2">
              <For each={PRESET_TAG_COLORS}>
                {(colorOption) => (
                  <button
                    type="button"
                    onClick={() => setSelectedColor(colorOption.value)}
                    class={cn(
                      "h-8 w-8 rounded-md border-2 transition-all",
                      selectedColor() === colorOption.value
                        ? "border-foreground scale-110"
                        : "border-transparent hover:scale-105"
                    )}
                    style={{ "background-color": colorOption.value }}
                    aria-label={colorOption.name}
                    title={colorOption.name}
                  />
                )}
              </For>
            </div>
          </div>

          <div class="flex gap-2">
            <Button
              type="button"
              onClick={handleCreateTag}
              disabled={isCreating()}
              class="flex-1"
            >
              {isCreating() ? "Creating..." : "Create"}
            </Button>
            <Button
              type="button"
              variant="outline"
              onClick={() => {
                setIsCreateMode(false);
                setInputValue("");
                setIsAutocompleteOpen(false);
              }}
              class="flex-1"
            >
              Cancel
            </Button>
          </div>
        </div>
      </Show>
    </div>
  );
}
