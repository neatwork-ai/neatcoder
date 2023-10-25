import Node from "./Node";
import defaultConverters from "./fromDelta.converters";

export default function deltaToMarkdown(
  ops: any[],
  converters = defaultConverters
): string {
  return trimEnd(convert(ops, converters).render()) + "\n";
}

function convert(ops: any[], converters: any): Node {
  let group: any,
    activeInline: any = {};
  const root = new Node();

  let el: Node = new Node(["", ""]);
  let line: Node = new Node(["", ""]);

  function newLine() {
    el = line = new Node(["", "\n"]);
    root.append(line);
    activeInline = {};
  }

  newLine();
  let beginningOfLine: boolean = false; // or true, depending on your logic

  for (let i = 0; i < ops.length; i++) {
    const op = ops[i];

    if (typeof op.insert === "object") {
      for (let k in op.insert) {
        if (converters.embed[k]) {
          applyInlineAttributes(op.attributes);
          converters.embed[k].call(el, op.insert[k], op.attributes);
        }
      }
    } else {
      const lines = op.insert.split("\n");

      if (hasBlockLevelAttribute(op.attributes, converters)) {
        for (let j = 1; j < lines.length; j++) {
          for (const attr in op.attributes) {
            if (converters.block[attr]) {
              let fn = converters.block[attr];
              if (typeof fn === "object") {
                if (group && group.type !== attr) {
                  group = null;
                }
                if (!group && fn.group) {
                  group = {
                    el: fn.group(),
                    type: attr,
                    value: op.attributes[attr],
                    distance: 0,
                  };
                  root.append(group.el);
                }

                if (group) {
                  group.el.append(line);
                  group.distance = 0;
                }
                fn = fn.line;
              }

              fn.call(line, op.attributes, group);
              newLine();
              break;
            }
          }
        }
        beginningOfLine = true;
      } else {
        for (let l = 0; l < lines.length; l++) {
          if ((l > 0 || beginningOfLine) && group && ++group.distance >= 2) {
            group = null;
          }
          applyInlineAttributes(
            op.attributes,
            ops[i + 1] && ops[i + 1].attributes
          );
          el.append(lines[l]);
          if (l < lines.length - 1) {
            newLine();
          }
        }
        beginningOfLine = false;
      }
    }
  }

  return root;

  function applyInlineAttributes(attrs: any, next?: any) {
    const first: string[] = [],
      then: string[] = [];
    attrs = attrs || {};

    let tag = el,
      seen: any = {};
    while (tag._format) {
      seen[tag._format] = true;
      if (!attrs[tag._format]) {
        for (const k in seen) {
          delete activeInline[k];
        }
        el = tag.parent()!;
      }

      tag = tag.parent()!;
    }

    for (const attr in attrs) {
      if (converters.inline[attr]) {
        if (activeInline[attr]) {
          if (activeInline[attr] === attrs[attr]) {
            continue;
          }
        }

        if (next && attrs[attr] === next[attr]) {
          first.push(attr);
        } else {
          then.push(attr);
        }
        activeInline[attr] = attrs[attr];
      }
    }

    first.forEach(apply);
    then.forEach(apply);

    function apply(fmt: string) {
      const converterOutput: string | [string, string] | Node =
        converters.inline[fmt].call(null, attrs[fmt]);

      let newEl: Node | string | [string, string];

      if (Array.isArray(converterOutput)) {
        newEl = new Node(converterOutput);
      } else if (typeof converterOutput === "string") {
        newEl = converterOutput;
      } else if (converterOutput instanceof Node) {
        newEl = converterOutput;
      } else {
        throw new Error("Unexpected converter output type");
      }

      if (newEl instanceof Node) {
        newEl._format = fmt;
        el.append(newEl);
        el = newEl;
      } else {
        el.append(newEl); // Assuming Node has a method to handle appending strings or [string, string]
      }
    }
  }
}

function hasBlockLevelAttribute(attrs: any, converters: any): boolean {
  for (const k in attrs) {
    if (Object.keys(converters.block).includes(k)) {
      return true;
    }
  }
  return false;
}

function trimEnd(str: string): string {
  return str.replace(/\s+$/, "");
}
