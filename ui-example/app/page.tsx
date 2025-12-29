"use client"

import { useState } from "react"
import { TaskPool } from "@/components/task-pool"
import { TaskQueue } from "@/components/task-queue"
import type { Task } from "@/types/task"

export default function TaskManagementPage() {
  const [poolTasks, setPoolTasks] = useState<Task[]>([
    {
      id: "1",
      title: "プロジェクト初期設定",
      status: "todo",
      children: [
        { id: "1-1", title: "Next.js環境構築", status: "completed" },
        { id: "1-2", title: "UIコンポーネント導入", status: "completed" },
        { id: "1-3", title: "データベース設計", status: "in-progress" },
      ],
    },
    {
      id: "2",
      title: "認証機能の実装",
      status: "todo",
      children: [
        { id: "2-1", title: "ログイン画面作成", status: "todo" },
        { id: "2-2", title: "JWT認証実装", status: "todo" },
      ],
    },
    {
      id: "3",
      title: "API開発",
      status: "todo",
      children: [
        { id: "3-1", title: "RESTful API設計", status: "todo" },
        { id: "3-2", title: "エンドポイント実装", status: "todo" },
        { id: "3-3", title: "バリデーション追加", status: "todo" },
      ],
    },
    {
      id: "4",
      title: "フロントエンド開発",
      status: "todo",
    },
    {
      id: "5",
      title: "テスト実装",
      status: "todo",
      children: [
        { id: "5-1", title: "ユニットテスト", status: "todo" },
        { id: "5-2", title: "E2Eテスト", status: "todo" },
      ],
    },
  ])

  const [queueTasks, setQueueTasks] = useState<Task[]>([
    { id: "q1", title: "データベース設計", status: "in-progress" },
    { id: "q2", title: "タスク管理画面のUI作成", status: "in-progress" },
  ])

  const moveToQueue = (task: Task) => {
    setQueueTasks([...queueTasks, { ...task, status: "in-progress" }])
  }

  const removeFromQueue = (taskId: string) => {
    setQueueTasks(queueTasks.filter((task) => task.id !== taskId))
  }

  const completeTask = (taskId: string) => {
    setQueueTasks(queueTasks.filter((task) => task.id !== taskId))
    setPoolTasks(
      poolTasks.map((task) =>
        task.id === taskId
          ? { ...task, status: "completed" }
          : {
              ...task,
              children: task.children?.map((child) =>
                child.id === taskId ? { ...child, status: "completed" } : child,
              ),
            },
      ),
    )
  }

  const queueTaskIds = new Set(queueTasks.map((task) => task.id))

  return (
    <div className="flex h-[calc(100vh-3.5rem)] bg-background">
      <TaskPool tasks={poolTasks} onMoveToQueue={moveToQueue} queueTaskIds={queueTaskIds} />
      <TaskQueue tasks={queueTasks} onRemove={removeFromQueue} onComplete={completeTask} />
    </div>
  )
}
