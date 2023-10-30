import { postProcessCodeBlocks } from "../quillToMarkdown/postProcess";

describe("postProcessCodeBlocks function", () => {
  it("should combine consecutive code-block ops into a single op", () => {
    const inputOps = [
      {
        insert: "pub my_function() {",
      },
      {
        attributes: {
          "code-block": true,
        },
        insert: "\n",
      },
      {
        insert: '  println!("Hello");',
      },
      {
        attributes: {
          "code-block": true,
        },
        insert: "\n",
      },
      {
        insert: "}",
      },
      {
        attributes: {
          "code-block": true,
        },
        insert: "\n",
      },
    ];

    const expectedOps = [
      {
        insert: 'pub my_function() {\n  println!("Hello");\n}',
        attributes: {
          "code-block": true,
        },
      },
    ];

    const resultOps = postProcessCodeBlocks(inputOps);
    expect(resultOps).toEqual(expectedOps);
  });
});
