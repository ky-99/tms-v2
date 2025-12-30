import { createSignal } from "solid-js";

export type ToastCategory = "network" | "validation" | "server";

export interface ToastMessage {
  id: string;
  category: ToastCategory;
  message: string;
  timestamp: number;
}

const [toasts, setToasts] = createSignal<ToastMessage[]>([]);

export const toastStore = {
  toasts,

  showError: (category: ToastCategory, message: string) => {
    const id = `toast-${Date.now()}-${Math.random()}`;
    const toast: ToastMessage = {
      id,
      category,
      message,
      timestamp: Date.now(),
    };

    setToasts([...toasts(), toast]);

    // 3秒後に自動削除
    setTimeout(() => {
      setToasts((prev) => prev.filter((t) => t.id !== id));
    }, 3000);
  },

  dismiss: (id: string) => {
    setToasts((prev) => prev.filter((t) => t.id !== id));
  },
};
