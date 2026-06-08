type WindowStates = "open" | "closed" | "minimized";
class MenuItem {
  id: number;
  name: string;
  window_state: WindowStates;

  constructor(id: number, name: string) {
    this.id = id;
    this.name = name;
    this.window_state = "closed";
  }

  openWindow(): void {
    this.window_state = "open";
  }

  minimizeWindow(): void {
    this.window_state = "minimized";
  }

  closeWindow(): void {
    this.window_state = "closed";
  }

  getState(): WindowStates {
    return this.window_state;
  }
}

export { MenuItem };
export type { WindowStates };
