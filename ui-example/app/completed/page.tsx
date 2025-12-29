"use client"

import { useState, useMemo } from "react"
import { CheckCircle2, Search } from "lucide-react"
import { Card } from "@/components/ui/card"
import { Input } from "@/components/ui/input"

// Mock data - replace with actual data from your state management
const completedTasks = [
  {
    id: "1-1",
    title: "Next.js environment setup",
    description: "Set up Next.js development environment with TypeScript",
    completedAt: "2024-01-20T14:30:00",
  },
  {
    id: "1-2",
    title: "UI component integration",
    description: "Integrate shadcn/ui components library",
    completedAt: "2024-01-20T16:45:00",
  },
  {
    id: "2-1",
    title: "Database connection",
    description: "Connect to PostgreSQL database",
    completedAt: "2024-01-19T10:15:00",
  },
  {
    id: "2-2",
    title: "Authentication setup",
    description: "Implement user authentication system",
    completedAt: "2024-01-19T15:20:00",
  },
  {
    id: "3-1",
    title: "API routes creation",
    description: "Create REST API endpoints",
    completedAt: "2024-01-18T09:00:00",
  },
]

function groupTasksByDate(tasks: typeof completedTasks) {
  const groups = new Map<string, typeof completedTasks>()

  tasks.forEach((task) => {
    const date = new Date(task.archivedAt).toDateString()
    if (!groups.has(date)) {
      groups.set(date, [])
    }
    groups.get(date)!.push(task)
  })

  return Array.from(groups.entries()).map(([date, tasks]) => ({
    date,
    tasks: tasks.sort((a, b) => new Date(b.completedAt).getTime() - new Date(a.completedAt).getTime()),
  }))
}

export default function CompletedPage() {
  const [searchQuery, setSearchQuery] = useState("")

  const filteredAndGroupedTasks = useMemo(() => {
    const filtered = completedTasks.filter(
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
            <CheckCircle2 className="h-6 w-6 text-primary" />
            <h1 className="text-2xl font-semibold text-foreground">Completed Tasks</h1>
          </div>

          <div className="relative">
            <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            <Input
              type="text"
              placeholder="Search completed tasks..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-9"
            />
          </div>
        </div>

        {filteredAndGroupedTasks.length === 0 ? (
          <Card className="p-8 text-center">
            <p className="text-muted-foreground">{searchQuery ? "No tasks found" : "No completed tasks yet"}</p>
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
                      <div className="absolute -left-[27px] top-2 h-3 w-3 rounded-full border-2 border-background bg-primary" />

                      <Card className="border-border bg-card p-4 transition-colors hover:bg-secondary/30">
                        <div className="flex items-start justify-between gap-4">
                          <div className="flex-1">
                            <div className="flex items-center gap-2">
                              <CheckCircle2 className="h-4 w-4 text-primary" />
                              <h3 className="font-medium text-foreground">{task.title}</h3>
                            </div>
                            <p className="mt-1 text-sm text-muted-foreground">{task.description}</p>
                            <time className="mt-2 block text-xs text-muted-foreground">
                              {new Date(task.completedAt).toLocaleTimeString("en-US", {
                                hour: "2-digit",
                                minute: "2-digit",
                              })}
                            </time>
                          </div>
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
