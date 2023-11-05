const codeTheme: { [key: string]: React.CSSProperties } = {
  'code[class*="language-"]': {
    color: "white",
    background: "none",
    // textShadow: "0 -.1em .2em black",
    fontFamily: "Consolas, Monaco, 'Andale Mono', 'Ubuntu Mono', monospace",
    fontSize: "1em",
    textAlign: "left",
    whiteSpace: "pre",
    wordSpacing: "normal",
    wordBreak: "normal",
    wordWrap: "normal",
    lineHeight: "1.5",
    MozTabSize: "4",
    OTabSize: "4",
    tabSize: "4",
    WebkitHyphens: "none",
    MozHyphens: "none",
    msHyphens: "none",
    hyphens: "none",
  },
  'pre[class*="language-"]': {
    color: "white",
    background: "hsl(30, 20%, 25%)",
    // textShadow: "0 -.1em .2em black",
    fontFamily: "Consolas, Monaco, 'Andale Mono', 'Ubuntu Mono', monospace",
    fontSize: "1em",
    textAlign: "left",
    whiteSpace: "pre",
    wordSpacing: "normal",
    wordBreak: "normal",
    wordWrap: "normal",
    lineHeight: "1.5",
    MozTabSize: "4",
    OTabSize: "4",
    tabSize: "4",
    WebkitHyphens: "none",
    MozHyphens: "none",
    msHyphens: "none",
    hyphens: "none",
    padding: "1em",
    margin: ".5em 0",
    overflow: "auto",
    border: ".3em solid hsl(30, 20%, 40%)",
    borderRadius: ".5em",
    // boxShadow: "1px 1px .5em black inset",
  },
  ':not(pre) > code[class*="language-"]': {
    background: "hsl(30, 20%, 25%)",
    padding: ".15em .2em .05em",
    borderRadius: ".3em",
    border: ".13em solid hsl(30, 20%, 40%)",
    // boxShadow: "1px 1px .3em -.1em black inset",
    whiteSpace: "normal",
  },
  comment: {
    color: "hsla(0, 0%, 100%, .5)", // OK
  },
  prolog: {
    color: "hsl(30, 20%, 50%)",
  },
  doctype: {
    color: "hsl(30, 20%, 50%)",
  },
  cdata: {
    color: "hsl(30, 20%, 50%)",
  },
  punctuation: {
    opacity: ".7",
  },
  namespace: {
    opacity: ".7",
  },
  property: {
    color: "hsl(350, 40%, 70%)",
  },
  tag: {
    color: "hsl(350, 40%, 70%)",
  },
  boolean: {
    color: "hsl(350, 40%, 70%)",
  },
  number: {
    color: "hsl(350, 40%, 70%)",
  },
  constant: {
    color: "hsl(350, 40%, 70%)",
  },
  symbol: {
    color: "hsl(350, 40%, 70%)",
  },
  selector: {
    color: "hsl(75, 70%, 60%)",
  },
  "attr-name": {
    color: "#df3079", // OK - I suppose this is akin to attr
  },
  string: {
    color: "#00a67d", // OK
  },
  char: {
    color: "hsl(75, 70%, 60%)",
  },
  builtin: {
    color: "#e9950c", // OK
  },
  inserted: {
    color: "hsl(75, 70%, 60%)",
  },
  operator: {
    color: "hsl(40, 90%, 60%)",
  },
  entity: {
    color: "hsl(40, 90%, 60%)",
    cursor: "help",
  },
  url: {
    color: "hsl(40, 90%, 60%)",
  },
  ".language-css .token.string": {
    color: "hsl(40, 90%, 60%)",
  },
  ".token.imports": {
    color: "#2e95d3", // Blue
  },
  ".style .token.string": {
    color: "hsl(40, 90%, 60%)",
  },
  variable: {
    color: "#df3079", // OK
  },
  atrule: {
    color: "hsl(350, 40%, 70%)",
  },
  "attr-value": {
    color: "hsl(350, 40%, 70%)",
  },
  keyword: {
    color: "#2e95d3", // OK
  },
  regex: {
    color: "#e90",
  },
  important: {
    color: "#e90",
    fontWeight: "bold",
  },
  bold: {
    fontWeight: "bold",
  },
  italic: {
    fontStyle: "italic",
  },
  deleted: {
    color: "red",
  },
};

export default codeTheme;
