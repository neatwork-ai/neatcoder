import Node from "./Node";
import { encodeLink } from "./Url";

interface Attributes {
  list?: string;
  header?: number;
}

interface Group {
  count?: number;
}

const converters = {
  embed: {
    image(this: Node, src: string) {
      this.append("![](" + encodeLink(src) + ")");
    },
    // Not a default Quill feature, converts custom divider embed blot added when
    // creating quill editor instance.
    // See https://quilljs.com/guides/cloning-medium-with-parchment/#dividers
    thematic_break(this: Node) {
      this.open = "\n---\n" + this.open;
    },
  },

  inline: {
    italic() {
      return ["_", "_"];
    },
    bold() {
      return ["**", "**"];
    },
    link(url: string) {
      return ["[", "](" + url + ")"];
    },
    code() {
      return ["`", "`"];
    },
  },

  block: {
    header(this: Node, { header }: Attributes) {
      this.open = "#".repeat(header!) + " " + this.open;
    },
    blockquote(this: Node) {
      this.open = "> " + this.open;
    },
    list: {
      group() {
        return new Node(["", "\n"]);
      },
      line(this: Node, attrs: Attributes, group: Group) {
        if (attrs.list === "bullet") {
          this.open = "- " + this.open;
        } else if (attrs.list === "checked") {
          this.open = "- [x] " + this.open;
        } else if (attrs.list === "unchecked") {
          this.open = "- [ ] " + this.open;
        } else if (attrs.list === "ordered") {
          group.count = group.count || 0;
          const count = ++group.count;
          this.open = count + ". " + this.open;
        }
      },
    },
    "code-block"(this: Node) {
      console.log("Code block converter called");
      this.open = "```\n" + this.open;
    },
  },
};

export default converters;
