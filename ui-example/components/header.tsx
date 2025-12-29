"use client"

import Link from "next/link"
import { usePathname } from "next/navigation"
import { Archive, CheckCircle2, ListTodo } from "lucide-react"

export function Header() {
  const pathname = usePathname()

  return (
    <header className="border-b border-border bg-background">
      <nav className="flex h-12 items-end px-4">
        <Link href="/" className="group">
          <div
            className={`flex items-center gap-2 px-4 pb-2 pt-3 rounded-t-lg transition-colors relative ${
              pathname === "/"
                ? "bg-card text-primary border-t-2 border-t-primary"
                : "bg-background/50 text-muted-foreground hover:bg-card/50 hover:text-foreground"
            }`}
          >
            <ListTodo className="h-4 w-4" />
            <span className="text-sm font-medium">Tasks</span>
            {pathname === "/" && <div className="absolute bottom-0 left-0 right-0 h-[2px] bg-card" />}
          </div>
        </Link>
        <Link href="/completed" className="group -ml-2">
          <div
            className={`flex items-center gap-2 px-4 pb-2 pt-3 rounded-t-lg transition-colors relative ${
              pathname === "/completed"
                ? "bg-card text-primary border-t-2 border-t-primary"
                : "bg-background/50 text-muted-foreground hover:bg-card/50 hover:text-foreground"
            }`}
          >
            <CheckCircle2 className="h-4 w-4" />
            <span className="text-sm font-medium">Completed</span>
            {pathname === "/completed" && <div className="absolute bottom-0 left-0 right-0 h-[2px] bg-card" />}
          </div>
        </Link>
        <Link href="/archive" className="group -ml-2">
          <div
            className={`flex items-center gap-2 px-4 pb-2 pt-3 rounded-t-lg transition-colors relative ${
              pathname === "/archive"
                ? "bg-card text-primary border-t-2 border-t-primary"
                : "bg-background/50 text-muted-foreground hover:bg-card/50 hover:text-foreground"
            }`}
          >
            <Archive className="h-4 w-4" />
            <span className="text-sm font-medium">Archive</span>
            {pathname === "/archive" && <div className="absolute bottom-0 left-0 right-0 h-[2px] bg-card" />}
          </div>
        </Link>
      </nav>
    </header>
  )
}
