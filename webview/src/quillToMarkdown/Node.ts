import isArray from "lodash/isArray";
import isString from "lodash/isString";
import pull from "lodash/pull";

let id = 0;

class Node {
  id: number;
  open?: string;
  close?: string;
  text?: string;
  children: Node[];
  private _parent?: Node;
  _format?: string;

  constructor(data?: string | [string, string]) {
    this.id = ++id;

    if (isArray(data)) {
      this.open = data[0];
      this.close = data[1];
    } else if (isString(data)) {
      this.text = data;
    }

    this.children = [];
  }

  append(e: string | Node): void {
    if (!(e instanceof Node)) {
      e = new Node(e);
    }

    if (e._parent) {
      pull(e._parent.children, e);
    }

    e._parent = this;
    this.children = this.children.concat(e);
  }

  render(): string {
    let text = "";

    if (this.open) {
      text += this.open;
    }

    if (this.text) {
      text += this.text;
    }

    for (let i = 0; i < this.children.length; i++) {
      text += this.children[i].render();
    }

    if (this.close) {
      text += this.close;
    }

    return text;
  }

  parent(): Node | undefined {
    return this._parent;
  }
}

export default Node;
