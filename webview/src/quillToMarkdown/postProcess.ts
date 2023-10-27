export function postProcessCodeBlocks(ops: any[]): any[] {
  const newOps = [];
  let buffer: any[] = [];
  let inCodeBlock = false;

  for (const op of ops) {
    if (op.attributes && op.attributes["code-block"]) {
      inCodeBlock = true;
      buffer.push(op);
    } else {
      if (inCodeBlock) {
        // We've reached the end of the code block
        newOps.push({
          insert: buffer.map((b) => b.insert).join(""),
          attributes: { "code-block": true },
        });
        buffer = [];
        inCodeBlock = false;
      }
      newOps.push(op);
    }
  }

  // If we have any remaining buffered operations, add them to the new operations
  if (buffer.length) {
    newOps.push({
      insert: buffer.map((b) => b.insert).join(""),
      attributes: { "code-block": true },
    });
  }

  return newOps;
}
