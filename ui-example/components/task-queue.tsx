"use client"

import { Check, X, Circle, AlertCircle } from "lucide-react"
import { Button } from "@/components/ui/button"
import type { Task } from "@/types/task"
import { cn } from "@/lib/utils"

interface TaskQueueProps {
  tasks: Task[]
  onRemove: (taskId: string) => void
  onComplete: (taskId: string) => void
}

export function TaskQueue({ tasks, onRemove, onComplete }: TaskQueueProps) {
  return (
    <div className="flex w-80 flex-col border-l border-border">
      <div className="border-b border-border bg-card px-6 py-4">
        <h2 className="text-xl font-semibold text-foreground">Task Queue</h2>
        <p className="mt-1 text-sm text-muted-foreground">Active: {tasks.length} tasks</p>
      </div>

      <div className="flex-1 overflow-y-auto p-4">
        {tasks.length === 0 ? (
          <div className="flex h-64 items-center justify-center rounded-lg border-2 border-dashed border-border">
            <p className="text-sm text-muted-foreground">Add tasks from the pool</p>
          </div>
        ) : (
          <div className="space-y-2">
            {tasks.map((task, index) => (
              <div
                key={task.id}
                className={cn(
                  "group flex items-center gap-3 rounded-lg border bg-card p-3 transition-all",
                  index === 0 ? "border-primary bg-primary/5 shadow-sm" : "border-border hover:bg-secondary/50",
                )}
              >
                {/* Status icon */}
                {index === 0 ? (
                  <AlertCircle className="h-4 w-4 shrink-0 text-primary" />
                ) : (
                  <Circle className="h-4 w-4 shrink-0 text-muted-foreground" />
                )}

                {/* Task title and position */}
                <div className="flex-1 min-w-0">
                  <p className="truncate text-sm font-medium text-foreground">{task.title}</p>
                  {index === 0 ? (
                    <p className="text-xs text-primary font-medium">In Progress</p>
                  ) : (
                    <p className="text-xs text-muted-foreground">Waiting ({index})</p>
                  )}
                </div>

                {/* Action buttons */}
                <div className="flex gap-1 shrink-0">
                  <Button
                    size="sm"
                    onClick={() => onComplete(task.id)}
                    className="h-8 w-8 p-0"
                    disabled={index !== 0}
                    title="Complete"
                  >
                    <Check className="h-4 w-4" />
                  </Button>
                  <Button
                    size="sm"
                    variant="ghost"
                    onClick={() => onRemove(task.id)}
                    className="h-8 w-8 p-0 text-destructive hover:bg-destructive/10 hover:text-destructive"
                    title="Remove"
                  >
                    <X className="h-4 w-4" />
                  </Button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  )
}
