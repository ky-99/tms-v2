import { createSignal, For, Show, onMount } from "solid-js";
import { Button } from "../components/Button";
import { Dialog } from "../components/Dialog";
import { ConfirmDialog } from "../components/ConfirmDialog";
import { Input } from "../components/Input";
import { TagInput } from "../components/TagInput";
import { tagsApi } from "../api/tags";
import type { Tag, CreateTagRequest, UpdateTagRequest } from "../types/tag";
import { DropdownMenu } from "../components/DropdownMenu";
import { truncateText } from "../lib/utils";
import { useSearchShortcut } from "../hooks/useSearchShortcut";

// Icon components
function SearchIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <circle cx="11" cy="11" r="8" />
      <path d="m21 21-4.35-4.35" />
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

function PencilIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" />
    </svg>
  );
}

function Trash2Icon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
      <line x1="10" y1="11" x2="10" y2="17" />
      <line x1="14" y1="11" x2="14" y2="17" />
    </svg>
  );
}

/**
 * タイムスタンプサフィックスを生成 (YYYYMMDD_HHmmss 形式)
 * 例: "20251230_153045"
 */
function generateTimestampSuffix(): string {
  const now = new Date();
  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, '0');
  const day = String(now.getDate()).padStart(2, '0');
  const hours = String(now.getHours()).padStart(2, '0');
  const minutes = String(now.getMinutes()).padStart(2, '0');
  const seconds = String(now.getSeconds()).padStart(2, '0');

  return `${year}${month}${day}_${hours}${minutes}${seconds}`;
}

export function TagManagementPage() {
  const [tags, setTags] = createSignal<Tag[]>([]);
  const [loading, setLoading] = createSignal(true);
  const [error, setError] = createSignal<string | null>(null);
  const [searchQuery, setSearchQuery] = createSignal("");
  const [searchInputRef, setSearchInputRef] = createSignal<HTMLInputElement | undefined>();

  // Create dialog
  const [showCreateDialog, setShowCreateDialog] = createSignal(false);
  const [newTagName, setNewTagName] = createSignal("");
  const [newTagColor, setNewTagColor] = createSignal("#3b82f6");

  // Edit dialog
  const [showEditDialog, setShowEditDialog] = createSignal(false);
  const [editingTag, setEditingTag] = createSignal<Tag | null>(null);
  const [editTagName, setEditTagName] = createSignal("");
  const [editTagColor, setEditTagColor] = createSignal("");

  // Delete confirmation
  const [showDeleteConfirm, setShowDeleteConfirm] = createSignal(false);
  const [deletingTag, setDeletingTag] = createSignal<Tag | null>(null);

  // Load tags
  const loadTags = async () => {
    setLoading(true);
    setError(null);
    try {
      const tagList = await tagsApi.list();
      setTags(tagList);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  };

  useSearchShortcut({
    getSearchInputRef: searchInputRef,
  });

  onMount(() => {
    loadTags();
  });

  // Create tag
  const handleCreate = async () => {
    const name = newTagName().trim();
    if (!name) return;

    try {
      const request: CreateTagRequest = {
        name,
        color: newTagColor(),
      };
      await tagsApi.create(request);
      setShowCreateDialog(false);
      setNewTagName("");
      setNewTagColor("#3b82f6");
      await loadTags();
    } catch (err) {
      alert(`Failed to create tag: ${err instanceof Error ? err.message : String(err)}`);
    }
  };

  // Edit tag
  const handleEditClick = (tag: Tag) => {
    setEditingTag(tag);
    setEditTagName(tag.name);
    setEditTagColor(tag.color);
    setShowEditDialog(true);
  };

  const handleUpdate = async () => {
    const tag = editingTag();
    if (!tag) return;

    const name = editTagName().trim();
    if (!name) return;

    try {
      const request: UpdateTagRequest = {
        name,
        color: editTagColor(),
      };
      await tagsApi.update(tag.id, request);
      setShowEditDialog(false);
      setEditingTag(null);
      await loadTags();
    } catch (err) {
      alert(`Failed to update tag: ${err instanceof Error ? err.message : String(err)}`);
    }
  };

  // Delete tag
  const handleDeleteClick = (tag: Tag) => {
    setDeletingTag(tag);
    setShowDeleteConfirm(true);
  };

  const handleDelete = async () => {
    const tag = deletingTag();
    if (!tag) return;

    try {
      await tagsApi.delete(tag.id);
      setShowDeleteConfirm(false);
      setDeletingTag(null);
      await loadTags();
    } catch (err) {
      alert(`Failed to delete tag: ${err instanceof Error ? err.message : String(err)}`);
    }
  };

  // Filter tags by search query
  const filteredTags = () => {
    const query = searchQuery().toLowerCase().trim();
    if (!query) return tags();
    return tags().filter(tag => tag.name.toLowerCase().includes(query));
  };

  /**
   * タグを複製する
   * - 複製されたタグは `{originalName}_YYYYMMDD_HHmmss` の形式で名前が付けられる
   * - 色とメタデータは元のタグと同じ
   */
  const handleDuplicate = async (tag: Tag) => {
    try {
      const timestamp = generateTimestampSuffix();
      const request: CreateTagRequest = {
        name: `${tag.name}_${timestamp}`,
        color: tag.color,
      };
      await tagsApi.create(request);
      await loadTags(); // リストを再読み込み
    } catch (err) {
      alert(`Failed to duplicate tag: ${err instanceof Error ? err.message : String(err)}`);
    }
  };

  return (
    <div class="flex flex-col h-full bg-background">
      {/* Content */}
      <div class="flex-1 overflow-y-auto p-6">
        {/* Search bar and New Tag button */}
        <div class="flex items-center gap-4 mb-6">
          <div class="relative flex-1">
            <div class="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground">
              <SearchIcon />
            </div>
            <Input
              ref={setSearchInputRef}
              type="text"
              placeholder="Search tags..."
              value={searchQuery()}
              onInput={(e) => setSearchQuery(e.currentTarget.value)}
              class="pl-9 bg-background"
            />
          </div>
          <Button onClick={() => setShowCreateDialog(true)} class="gap-2 shrink-0">
            <PlusIcon />
            New Tag
          </Button>
        </div>
        <Show when={loading()}>
          <div class="text-center text-muted-foreground py-8">Loading tags...</div>
        </Show>

        <Show when={error()}>
          <div class="text-center text-destructive py-8">{error()}</div>
        </Show>

        <Show when={!loading() && !error()}>
          <Show when={filteredTags().length === 0} fallback={
            <div class="bg-card border border-border rounded-lg overflow-hidden">
              <table class="w-full">
                <thead class="bg-secondary/50 border-b border-border">
                  <tr>
                    <th class="text-left px-4 py-3 text-sm font-medium text-foreground">Name</th>
                    <th class="text-left px-4 py-3 text-sm font-medium text-foreground">Color</th>
                    <th class="text-left px-4 py-3 text-sm font-medium text-foreground">Usage Count</th>
                    <th class="text-right px-4 py-3 text-sm font-medium text-foreground">Actions</th>
                  </tr>
                </thead>
                <tbody>
                  <For each={filteredTags()}>
                    {(tag) => (
                      <tr class="border-b border-border hover:bg-secondary/30 transition-colors">
                        <td class="px-4 py-3">
                          <span
                            class="inline-block px-2 py-1 text-xs font-medium rounded-md"
                            style={{
                              "background-color": `${tag.color}20`,
                              color: tag.color,
                            }}
                            title={tag.name}
                          >
                            {truncateText(tag.name, 40)}
                          </span>
                        </td>
                        <td class="px-4 py-3">
                          <div class="flex items-center gap-2">
                            <div
                              class="w-6 h-6 rounded-full border border-border"
                              style={{ "background-color": tag.color }}
                            />
                            <span class="text-sm text-muted-foreground">{tag.color}</span>
                          </div>
                        </td>
                        <td class="px-4 py-3 text-sm text-foreground">
                          {tag.usageCount || 0} task{tag.usageCount === 1 ? "" : "s"}
                        </td>
                        <td class="px-4 py-3">
                          <div class="flex justify-end">
                            <DropdownMenu
                              items={[
                                {
                                  label: "Edit",
                                  onClick: () => handleEditClick(tag),
                                  variant: "default"
                                },
                                {
                                  label: "Duplicate",
                                  onClick: () => handleDuplicate(tag),
                                  variant: "default"
                                },
                                {
                                  label: "Delete",
                                  onClick: () => handleDeleteClick(tag),
                                  variant: "destructive"
                                }
                              ]}
                            />
                          </div>
                        </td>
                      </tr>
                    )}
                  </For>
                </tbody>
              </table>
            </div>
          }>
            <div class="text-center text-muted-foreground py-12">
              <Show when={searchQuery().trim()} fallback={
                <>
                  <p class="text-lg mb-4">No tags yet</p>
                  <p class="text-sm mb-6">Create your first tag to get started</p>
                  <Button onClick={() => setShowCreateDialog(true)} class="gap-2">
                    <PlusIcon />
                    Create Tag
                  </Button>
                </>
              }>
                <p class="text-lg mb-4">No tags found</p>
                <p class="text-sm">No tags match "{searchQuery()}"</p>
              </Show>
            </div>
          </Show>
        </Show>
      </div>

      {/* Create Dialog */}
      <Dialog
        open={showCreateDialog()}
        onOpenChange={setShowCreateDialog}
        title="Create New Tag"
        description="Create a new tag to organize your tasks"
      >
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-foreground mb-2">Tag Name</label>
            <Input
              type="text"
              value={newTagName()}
              onInput={(e) => setNewTagName(e.currentTarget.value)}
              placeholder="Enter tag name"
              autofocus
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-foreground mb-2">Color</label>
            <div class="flex items-center gap-3">
              <input
                type="color"
                value={newTagColor()}
                onInput={(e) => setNewTagColor(e.currentTarget.value)}
                class="h-10 w-20 cursor-pointer rounded border border-input bg-background"
              />
              <span class="text-sm text-muted-foreground">{newTagColor()}</span>
            </div>
          </div>

          <div class="flex gap-2 justify-end">
            <Button variant="secondary" onClick={() => setShowCreateDialog(false)}>
              Cancel
            </Button>
            <Button onClick={handleCreate} disabled={!newTagName().trim()}>
              Create
            </Button>
          </div>
        </div>
      </Dialog>

      {/* Edit Dialog */}
      <Dialog
        open={showEditDialog()}
        onOpenChange={setShowEditDialog}
        title="Edit Tag"
        description="Update tag name or color"
      >
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-foreground mb-2">Tag Name</label>
            <Input
              type="text"
              value={editTagName()}
              onInput={(e) => setEditTagName(e.currentTarget.value)}
              placeholder="Enter tag name"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-foreground mb-2">Color</label>
            <div class="flex items-center gap-3">
              <input
                type="color"
                value={editTagColor()}
                onInput={(e) => setEditTagColor(e.currentTarget.value)}
                class="h-10 w-20 cursor-pointer rounded border border-input bg-background"
              />
              <span class="text-sm text-muted-foreground">{editTagColor()}</span>
            </div>
          </div>

          <div class="flex gap-2 justify-end">
            <Button variant="secondary" onClick={() => setShowEditDialog(false)}>
              Cancel
            </Button>
            <Button onClick={handleUpdate} disabled={!editTagName().trim()}>
              Update
            </Button>
          </div>
        </div>
      </Dialog>

      {/* Delete Confirmation */}
      <ConfirmDialog
        open={showDeleteConfirm()}
        onOpenChange={setShowDeleteConfirm}
        onConfirm={handleDelete}
        title="Delete Tag"
        description={
          deletingTag()
            ? `Are you sure you want to delete "${truncateText(deletingTag()!.name, 40)}"? ${
                (deletingTag()!.usageCount || 0) > 0
                  ? `This tag is currently used by ${deletingTag()!.usageCount} task${deletingTag()!.usageCount === 1 ? "" : "s"}. Deleting it will remove it from all tasks.`
                  : "This action cannot be undone."
              }`
            : ""
        }
        confirmText="Delete"
        destructive
      />
    </div>
  );
}
