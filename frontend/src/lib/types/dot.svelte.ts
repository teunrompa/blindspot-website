class DotObject {
  id = $state(0);
  d = $state("");
  x = $state(0);
  y = $state(0);
  scale = $state(1);
  width = $state(0);
  height = $state(0);
  offsetX = $state(0);
  offsetY = $state(0);

  constructor(id: number, d: string, bbox: DOMRect) {
    this.id = id;
    this.d = d;
    this.x = bbox.x;
    this.y = bbox.y;
    this.width = bbox.width;
    this.height = bbox.height;
  }

  centerX() {
    return this.x + this.width / 2;
  }
  centerY() {
    return this.y + this.height / 2;
  }
}

export { DotObject };
