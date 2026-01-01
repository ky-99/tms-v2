import { createSignal, For, Show } from "solid-js";
import { cn, truncateText } from "../lib/utils";
import { Button } from "./Button";
import { TagSelect } from "./TagSelect";
import type { Tag } from "../types/tag";
import { XIcon } from "./icons";

interface TagInputProps {
  selectedTags: string[]; // Array of tag names
  onTagsChange: (tags: string[]) => void;
  availableTags: Tag[];
  onCreateTag?: (name: string, color: string) => Promise<Tag>;
  placeholder?: string;
}

/**
 * タグ入力コンポーネント
 *
 * - チップ入力スタイルで既存タグを表示・削除
 * - オートコンプリートで既存タグを検索・追加
 * - インライン新規タグ作成（名前+Ark UIカラーピッカー）
 */
export function TagInput(props: TagInputProps) {
  const [isCreateMode, setIsCreateMode] = createSignal(false);
  const [newTagName, setNewTagName] = createSignal("");
  const [selectedColor, setSelectedColor] = createSignal("#3b82f6");
  const [isCreating, setIsCreating] = createSignal(false);

  // Toggle tag selection
  const toggleTag = (tagName: string) => {
    if (props.selectedTags.includes(tagName)) {
      // Remove tag
      props.onTagsChange(props.selectedTags.filter((t) => t !== tagName));
    } else {
      // Add tag
      props.onTagsChange([...props.selectedTags, tagName]);
    }
  };

  // Remove tag by name (for chip removal)
  const removeTag = (tagName: string) => {
    props.onTagsChange(props.selectedTags.filter((t) => t !== tagName));
  };

  // Open create mode
  const handleOpenCreateMode = () => {
    setIsCreateMode(true);
    setNewTagName("");
  };

  // Create new tag
  const handleCreateTag = async () => {
    const tagName = newTagName().trim();
    if (!tagName || !props.onCreateTag) return;

    setIsCreating(true);
    try {
      await props.onCreateTag(tagName, selectedColor());
      toggleTag(tagName);
      setNewTagName("");
      setSelectedColor("#3b82f6"); // Reset to default blue
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
                  title={tagName}
                >
                  <span>{truncateText(tagName, 30)}</span>
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

      {/* Tag selector using Kobalte */}
      <TagSelect
        selectedTags={props.selectedTags}
        availableTags={props.availableTags}
        onToggleTag={toggleTag}
        onOpenCreateMode={props.onCreateTag ? handleOpenCreateMode : undefined}
        placeholder={props.placeholder}
      />

      {/* Create tag form - one line inline layout */}
      <Show when={isCreateMode()}>
        <div class="flex items-center gap-2 p-2 rounded-md border border-border bg-card/50">
          <input
            type="text"
            value={newTagName()}
            onInput={(e) => setNewTagName(e.currentTarget.value)}
            placeholder="Tag name..."
            class="flex-1 px-2 py-1 border border-input rounded-md bg-background focus:outline-none focus:ring-1 focus:ring-ring text-sm placeholder:text-muted-foreground"
          />
          <input
            type="color"
            value={selectedColor()}
            onInput={(e) => setSelectedColor(e.currentTarget.value)}
            class="h-8 w-12 cursor-pointer rounded border border-input bg-background"
            title="Pick color"
          />
          <button
            type="button"
            onClick={() => {
              setIsCreateMode(false);
              setNewTagName("");
            }}
            class="px-2 py-1 rounded-md border border-input bg-background hover:bg-secondary transition-colors"
            title="Cancel"
          >
            <XIcon />
          </button>
          <button
            type="button"
            onClick={handleCreateTag}
            disabled={isCreating() || !newTagName().trim()}
            class="px-2 py-1 rounded-md bg-primary text-primary-foreground hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            title={isCreating() ? "Creating..." : "Create"}
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
              <path d="M5 13l4 4L19 7" />
            </svg>
          </button>
        </div>
      </Show>
    </div>
  );
}
