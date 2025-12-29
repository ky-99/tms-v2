"use client"

import { useState } from "react"
import {
  ChevronDown,
  ChevronRight,
  Plus,
  Circle,
  CheckCircle2,
  ArrowRight,
  Pencil,
  Trash2,
  Search,
  X,
} from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import type { Task } from "@/types/task"
import { cn } from "@/lib/utils"

interface TaskPoolProps {
  tasks: Task[]
  onMoveToQueue: (task: Task) => void
  queueTaskIds: Set<string>
}

export function TaskPool({ tasks, onMoveToQueue, queueTaskIds }: TaskPoolProps) {
  const [expandedTasks, setExpandedTasks] = useState<Set<string>>(new Set(["1", "2"]))
  const [searchQuery, setSearchQuery] = useState("")
  const [activeFilters, setActiveFilters] = useState<Set<Task["status"]>>(new Set())

  const toggleExpand = (taskId: string) => {
    const newExpanded = new Set(expandedTasks)
    if (newExpanded.has(taskId)) {
      newExpanded.delete(taskId)
    } else {
      newExpanded.add(taskId)
    }
    setExpandedTasks(newExpanded)
  }

  const toggleFilter = (status: Task["status"]) => {
    const newFilters = new Set(activeFilters)
    if (newFilters.has(status)) {
      newFilters.delete(status)
    } else {
      newFilters.add(status)
    }
    setActiveFilters(newFilters)
  }

  const filteredTasks = tasks.filter((task) => {
    const matchesSearch =
      task.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      task.children?.some((child) => child.title.toLowerCase().includes(searchQuery.toLowerCase()))
    const matchesFilter = activeFilters.size === 0 || activeFilters.has(task.status)
    return matchesSearch && matchesFilter
  })

  const getStatusIcon = (status: Task["status"]) => {
    switch (status) {
      case "completed":
        return <CheckCircle2 className="h-4 w-4 text-primary" />
      case "in-progress":
        return <Circle className="h-4 w-4 fill-primary text-primary" />
      default:
        return <Circle className="h-4 w-4 text-muted-foreground" />
    }
  }

  const calculateProgress = (task: Task): number => {
    if (!task.children || task.children.length === 0) return 0
    const completedChildren = task.children.filter((child) => child.status === "completed").length
    return Math.round((completedChildren / task.children.length) * 100)
  }

  const ProgressCircle = ({ progress }: { progress: number }) => {
    const radius = 8
    const circumference = 2 * Math.PI * radius
    const offset = circumference - (progress / 100) * circumference

    return (
      <div className="relative flex h-5 w-5 items-center justify-center">
        <svg className="h-5 w-5 -rotate-90" viewBox="0 0 20 20">
          <circle
            cx="10"
            cy="10"
            r={radius}
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            className="text-muted-foreground/30"
          />
          <circle
            cx="10"
            cy="10"
            r={radius}
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeDasharray={circumference}
            strokeDashoffset={offset}
            className="text-primary transition-all duration-300"
            strokeLinecap="round"
          />
        </svg>
        <span className="absolute text-[8px] font-semibold text-foreground">{progress}</span>
      </div>
    )
  }

  const handleEdit = (task: Task) => {
    console.log("Edit task:", task)
  }

  const handleDelete = (task: Task) => {
    console.log("Delete task:", task)
  }

  return (
    <div className="flex flex-1 flex-col border-r border-border">
      <div className="border-b border-border bg-card px-4 py-3 space-y-3">
        <div className="flex items-center gap-2">
          <div className="relative flex-1">
            <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            <Input
              type="text"
              placeholder="Search tasks..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-9 pr-9 bg-background"
            />
            {searchQuery && (
              <Button
                size="sm"
                variant="ghost"
                onClick={() => setSearchQuery("")}
                className="absolute right-1 top-1/2 h-7 w-7 -translate-y-1/2 p-0"
              >
                <X className="h-4 w-4" />
              </Button>
            )}
          </div>
          <Button size="sm" className="h-10 w-10 p-0 shrink-0" title="New Task">
            <Plus className="h-4 w-4" />
          </Button>
        </div>

        <div className="flex flex-wrap gap-2">
          <button
            onClick={() => toggleFilter("todo")}
            className={cn(
              "rounded-full px-3 py-1 text-xs font-medium transition-colors",
              activeFilters.has("todo")
                ? "bg-primary text-primary-foreground"
                : "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            )}
          >
            To Do
          </button>
          <button
            onClick={() => toggleFilter("in-progress")}
            className={cn(
              "rounded-full px-3 py-1 text-xs font-medium transition-colors",
              activeFilters.has("in-progress")
                ? "bg-primary text-primary-foreground"
                : "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            )}
          >
            In Progress
          </button>
          <button
            onClick={() => toggleFilter("completed")}
            className={cn(
              "rounded-full px-3 py-1 text-xs font-medium transition-colors",
              activeFilters.has("completed")
                ? "bg-primary text-primary-foreground"
                : "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            )}
          >
            Completed
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-4">
        <div className="space-y-2">
          {filteredTasks.map((task) => (
            <div key={task.id} className="space-y-1">
              <div
                onClick={() => {
                  if (task.children && task.children.length > 0) {
                    toggleExpand(task.id)
                  }
                }}
                className={cn(
                  "group flex items-center gap-3 rounded-lg bg-card p-3 transition-colors",
                  task.status === "completed" && "opacity-60",
                  queueTaskIds.has(task.id)
                    ? "bg-primary/10 border border-primary/20 hover:bg-primary/5"
                    : "hover:bg-secondary/50",
                  task.children && task.children.length > 0 && "cursor-pointer",
                )}
              >
                {task.children && task.children.length > 0 && (
                  <div className="text-muted-foreground">
                    {expandedTasks.has(task.id) ? (
                      <ChevronDown className="h-4 w-4" />
                    ) : (
                      <ChevronRight className="h-4 w-4" />
                    )}
                  </div>
                )}
                {(!task.children || task.children.length === 0) && <div className="w-4" />}

                {task.children && task.children.length > 0 ? (
                  <ProgressCircle progress={calculateProgress(task)} />
                ) : (
                  getStatusIcon(task.status)
                )}

                <span
                  className={cn(
                    "flex-1 text-sm font-medium text-foreground",
                    task.status === "completed" && "line-through",
                  )}
                >
                  {task.title}
                </span>

                <div className="flex items-center gap-1 opacity-0 transition-opacity group-hover:opacity-100">
                  <Button
                    size="sm"
                    variant="ghost"
                    onClick={(e) => {
                      e.stopPropagation()
                      handleEdit(task)
                    }}
                    className="h-8 w-8 p-0"
                    title="Edit Task"
                  >
                    <Pencil className="h-4 w-4" />
                  </Button>
                  <Button
                    size="sm"
                    variant="ghost"
                    onClick={(e) => {
                      e.stopPropagation()
                      handleDelete(task)
                    }}
                    className="h-8 w-8 p-0 text-destructive hover:text-destructive"
                    title="Delete Task"
                  >
                    <Trash2 className="h-4 w-4" />
                  </Button>
                  <Button
                    size="sm"
                    variant="ghost"
                    onClick={(e) => {
                      e.stopPropagation()
                      onMoveToQueue(task)
                    }}
                    className="h-8 w-8 p-0"
                    title="Add to Queue"
                    disabled={queueTaskIds.has(task.id)}
                  >
                    <ArrowRight className="h-4 w-4" />
                  </Button>
                </div>
              </div>

              {task.children && task.children.length > 0 && expandedTasks.has(task.id) && (
                <div className="ml-6 space-y-1 border-l-2 border-border pl-4">
                  {task.children.map((child) => (
                    <div
                      key={child.id}
                      className={cn(
                        "group flex items-center gap-3 rounded-lg bg-card p-2.5 transition-colors",
                        child.status === "completed" && "opacity-60",
                        queueTaskIds.has(child.id)
                          ? "bg-primary/10 border border-primary/20 hover:bg-primary/5"
                          : "hover:bg-secondary/50",
                      )}
                    >
                      {getStatusIcon(child.status)}
                      <span
                        className={cn("flex-1 text-sm text-foreground", child.status === "completed" && "line-through")}
                      >
                        {child.title}
                      </span>
                      <div className="flex items-center gap-1 opacity-0 transition-opacity group-hover:opacity-100">
                        <Button
                          size="sm"
                          variant="ghost"
                          onClick={(e) => {
                            e.stopPropagation()
                            handleEdit(child)
                          }}
                          className="h-8 w-8 p-0"
                          title="Edit Task"
                        >
                          <Pencil className="h-4 w-4" />
                        </Button>
                        <Button
                          size="sm"
                          variant="ghost"
                          onClick={(e) => {
                            e.stopPropagation()
                            handleDelete(child)
                          }}
                          className="h-8 w-8 p-0 text-destructive hover:text-destructive"
                          title="Delete Task"
                        >
                          <Trash2 className="h-4 w-4" />
                        </Button>
                        <Button
                          size="sm"
                          variant="ghost"
                          onClick={() => onMoveToQueue(child)}
                          className="h-8 w-8 p-0"
                          title="Add to Queue"
                          disabled={queueTaskIds.has(child.id)}
                        >
                          <ArrowRight className="h-4 w-4" />
                        </Button>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
