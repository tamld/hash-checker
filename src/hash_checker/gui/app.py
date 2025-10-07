"""Tkinter-based graphical interface for the Hash Checker project."""
from __future__ import annotations

import os
import sys
from pathlib import Path
from typing import Iterable

try:
    import tkinter as tk
    from tkinter import filedialog, messagebox, ttk
except Exception:  # pragma: no cover - Tk may be missing
    tk = None
    filedialog = None
    messagebox = None
    ttk = None

try:
    from tkinterdnd2 import DND_FILES, TkinterDnD
except Exception:  # pragma: no cover - optional dependency
    DND_FILES = None
    TkinterDnD = None

from ..core import verify_hash


class HashCheckerApp:
    """Simple drag-and-drop GUI for verifying file hashes."""

    def __init__(self, algorithms: Iterable[str]):
        if tk is None:
            raise RuntimeError("Tkinter is not available in this environment")

        base_cls = TkinterDnD.Tk if TkinterDnD and tk else tk.Tk
        self.root = base_cls()
        self.root.title("Hash Checker")
        self.root.geometry("600x340")
        self.root.resizable(True, False)
        self.algorithms = sorted({algo.lower() for algo in algorithms})

        self.file_var = tk.StringVar()
        self.expected_var = tk.StringVar()
        self.algorithm_var = tk.StringVar(value="auto")
        self.result_var = tk.StringVar()
        self.computed_var = tk.StringVar()

        self._configure_style()
        self._build_layout()

        if TkinterDnD and tk and self.root is not None:
            drop_target = self.drop_area
            drop_target.drop_target_register(DND_FILES)
            drop_target.dnd_bind("<<Drop>>", self._on_drop)

    def _configure_style(self) -> None:
        style = ttk.Style(self.root)
        try:
            style.theme_use("clam")
        except tk.TclError:  # pragma: no cover
            pass
        style.configure("Card.TFrame", padding=20)
        style.configure("Card.TLabel", font=("Segoe UI", 11))
        style.configure("CardHeading.TLabel", font=("Segoe UI", 12, "bold"))
        style.configure("Result.TLabel", font=("Segoe UI", 12, "bold"))

    def _build_layout(self) -> None:
        container = ttk.Frame(self.root, style="Card.TFrame")
        container.pack(fill=tk.BOTH, expand=True)

        heading = ttk.Label(
            container,
            text="Verify file integrity in three steps",
            style="CardHeading.TLabel",
        )
        heading.pack(anchor=tk.W, pady=(0, 12))

        file_row = ttk.Frame(container)
        file_row.pack(fill=tk.X, pady=4)
        ttk.Label(file_row, text="1. Choose file:", style="Card.TLabel").pack(side=tk.LEFT)
        entry = ttk.Entry(file_row, textvariable=self.file_var)
        entry.pack(side=tk.LEFT, fill=tk.X, expand=True, padx=8)
        ttk.Button(file_row, text="Browse...", command=self._browse_file).pack(side=tk.RIGHT)

        drop_label = ttk.Label(
            container,
            text="Drag & drop a file here",
            anchor="center",
        )
        drop_label.pack(fill=tk.X, pady=(6, 12))
        drop_label.configure(relief=tk.RIDGE, padding=16)
        self.drop_area = drop_label

        expected_row = ttk.Frame(container)
        expected_row.pack(fill=tk.X, pady=4)
        ttk.Label(expected_row, text="2. Expected hash:", style="Card.TLabel").pack(side=tk.LEFT)
        expected_entry = ttk.Entry(expected_row, textvariable=self.expected_var)
        expected_entry.pack(side=tk.LEFT, fill=tk.X, expand=True, padx=8)

        algo_row = ttk.Frame(container)
        algo_row.pack(fill=tk.X, pady=4)
        ttk.Label(algo_row, text="Algorithm:", style="Card.TLabel").pack(side=tk.LEFT)
        choices = ["auto"] + self.algorithms
        combo = ttk.Combobox(
            algo_row,
            textvariable=self.algorithm_var,
            values=choices,
            state="readonly",
        )
        combo.current(0)
        combo.pack(side=tk.LEFT, padx=8)

        button_row = ttk.Frame(container)
        button_row.pack(fill=tk.X, pady=12)
        ttk.Button(button_row, text="Verify", command=self._verify).pack(side=tk.LEFT)
        ttk.Button(button_row, text="Clear", command=self._clear).pack(side=tk.LEFT, padx=8)

        result_label = ttk.Label(
            container,
            textvariable=self.result_var,
            style="Result.TLabel",
        )
        result_label.pack(anchor=tk.W, pady=(0, 12))

        computed_row = ttk.Frame(container)
        computed_row.pack(fill=tk.X, pady=4)
        ttk.Label(computed_row, text="Computed hash:", style="Card.TLabel").pack(side=tk.LEFT)
        computed_entry = ttk.Entry(
            computed_row,
            textvariable=self.computed_var,
            state="readonly",
        )
        computed_entry.pack(side=tk.LEFT, fill=tk.X, expand=True, padx=8)
        ttk.Button(
            computed_row,
            text="Copy",
            command=lambda: self._copy_to_clipboard(self.computed_var.get()),
        ).pack(side=tk.LEFT)

    def _browse_file(self) -> None:
        if filedialog is None:
            if messagebox:
                messagebox.showerror("Error", "File dialog unavailable in this environment")
            return
        initial = self.file_var.get() or os.getcwd()
        filename = filedialog.askopenfilename(initialdir=initial)
        if filename:
            self.file_var.set(filename)

    def _normalize_path(self, path: str) -> str:
        cleaned = path.strip()
        if cleaned.startswith("{") and cleaned.endswith("}"):
            cleaned = cleaned[1:-1]
        return cleaned

    def _on_drop(self, event) -> None:  # pragma: no cover - UI interaction
        items = self.root.splitlist(event.data)
        if not items:
            return
        normalized = [self._normalize_path(item) for item in items]
        self.file_var.set(normalized[0])

    def _clear(self) -> None:
        self.file_var.set("")
        self.expected_var.set("")
        self.algorithm_var.set("auto")
        self.result_var.set("")
        self.computed_var.set("")

    def _copy_to_clipboard(self, value: str) -> None:
        if not value:
            return
        self.root.clipboard_clear()
        self.root.clipboard_append(value)
        self.root.update()  # keep clipboard contents after closing

    def _verify(self) -> None:
        file_path = Path(self.file_var.get())
        expected = self.expected_var.get()
        algo = self.algorithm_var.get()
        selected_algo = None if algo == "auto" else algo
        try:
            matches, computed = verify_hash(file_path, expected, selected_algo)
        except Exception as exc:  # pragma: no cover - user feedback
            self.result_var.set(f"Error: {exc}")
            self.computed_var.set("")
            if messagebox:
                messagebox.showerror("Verification failed", str(exc))
            return
        self.computed_var.set(computed)
        self.result_var.set("Match" if matches else "Mismatch")
        if matches and messagebox:
            messagebox.showinfo("Success", "Hashes match")
        elif messagebox:
            messagebox.showwarning("Mismatch", "Hashes do not match")

    def run(self) -> None:  # pragma: no cover - UI loop
        self.root.mainloop()


def launch_gui(algorithms: Iterable[str]) -> int:
    if tk is None:
        print("Tkinter is not available on this system. GUI mode cannot start.", file=sys.stderr)
        return 5
    app = HashCheckerApp(algorithms)
    app.run()
    return 0
