"use client"

import { useState, useMemo } from "react"
import { Archive, RotateCcw, Search } from "lucide-react"
import { Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"

// Mock data - replace with actual data from your state management
const archivedTasks = [
  {
    id: "a1",
    title: "Remove old features",
    description: "Clean up deprecated functionality from the codebase",
    archivedAt: "2024-01-20T11:30:00",
  },
  {
    id: "a2",
    title: "Legacy code refactoring",
    description: "Refactor old authentication module",
    archivedAt: "2024-01-20T13:15:00",
  },
  {
    id: "a3",
    title: "Outdated dependencies",
    description: "Remove unused npm packages",
    archivedAt: "2024-01-19T16:45:00",
  },
  {
    id: "a4",
    title: "Old UI components",
    description: "Archive replaced component library",
    archivedAt: "2024-01-18T14:20:00",
  },
]

function groupTasksByDate(tasks: typeof archivedTasks) {
  const groups = new Map<string, typeof archivedTasks>()

  tasks.forEach((task) => {
    const date = new Date(task.archivedAt).toDateString()
    if (!groups.has(date)) {
      groups.set(date, [])
    }
    groups.get(date)!.push(task)
  })

  return Array.from(groups.entries()).map(([date, tasks]) => ({
    date,
    tasks: tasks.sort((a, b) => new Date(b.archivedAt).getTime() - new Date(a.archivedAt).getTime()),
  }))
}

export default function ArchivePage() {
  const [searchQuery, setSearchQuery] = useState("")

  const handleRestore = (taskId: string) => {
    console.log("Restore task:", taskId)
    // Implement restore functionality
  }

  const filteredAndGroupedTasks = useMemo(() => {
    const filtered = archivedTasks.filter(
      (task) =>
        task.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        task.description.toLowerCase().includes(searchQuery.toLowerCase()),
    )
    return groupTasksByDate(filtered).sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())
  }, [searchQuery])

  return (
    <div className="h-[calc(100vh-3.5rem)] overflow-auto bg-background p-6">
      <div className="mx-auto max-w-4xl">
        <div className="mb-6">
          <div className="mb-4 flex items-center gap-3">
            <Archive className="h-6 w-6 text-primary" />
            <h1 className="text-2xl font-semibold text-foreground">Archived Tasks</h1>
          </div>

          <div className="relative">
            <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            <Input
              type="text"
              placeholder="Search archived tasks..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-9"
            />
          </div>
        </div>

        {filteredAndGroupedTasks.length === 0 ? (
          <Card className="p-8 text-center">
            <p className="text-muted-foreground">{searchQuery ? "No tasks found" : "No archived tasks"}</p>
          </Card>
        ) : (
          <div className="space-y-8">
            {filteredAndGroupedTasks.map(({ date, tasks }) => (
              <div key={date} className="relative">
                <div className="mb-4 flex items-center gap-3">
                  <div className="rounded-md bg-secondary px-3 py-1">
                    <time className="text-sm font-medium text-foreground">
                      {new Date(date).toLocaleDateString("en-US", {
                        month: "short",
                        day: "numeric",
                        year: "numeric",
                      })}
                    </time>
                  </div>
                  <div className="h-px flex-1 bg-border" />
                </div>

                <div className="space-y-3 border-l-2 border-border pl-6">
                  {tasks.map((task) => (
                    <div key={task.id} className="relative">
                      <div className="absolute -left-[27px] top-2 h-3 w-3 rounded-full border-2 border-background bg-muted-foreground" />

                      <Card className="border-border bg-card p-4 transition-colors hover:bg-secondary/30">
                        <div className="flex items-start justify-between gap-4">
                          <div className="flex-1">
                            <div className="flex items-center gap-2">
                              <Archive className="h-4 w-4 text-muted-foreground" />
                              <h3 className="font-medium text-foreground">{task.title}</h3>
                            </div>
                            <p className="mt-1 text-sm text-muted-foreground">{task.description}</p>
                            <time className="mt-2 block text-xs text-muted-foreground">
                              {new Date(task.archivedAt).toLocaleTimeString("en-US", {
                                hour: "2-digit",
                                minute: "2-digit",
                              })}
                            </time>
                          </div>
                          <Button
                            variant="ghost"
                            size="icon"
                            onClick={() => handleRestore(task.id)}
                            className="text-primary hover:bg-primary/10 hover:text-primary"
                            title="Restore task"
                          >
                            <RotateCcw className="h-4 w-4" />
                          </Button>
                        </div>
                      </Card>
                    </div>
                  ))}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  )
}
