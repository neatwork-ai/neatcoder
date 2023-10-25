import render from "./fromDelta";

test("renders inline format", () => {
  expect(
    render([
      {
        insert: "Hi ",
      },
      {
        attributes: {
          bold: true,
        },
        insert: "mom",
      },
    ])
  ).toEqual("Hi **mom**\n");
});

test("renders embed format", () => {
  expect(
    render([
      {
        insert: "LOOK AT THE KITTEN!\n",
      },
      {
        insert: { image: "https://placekitten.com/g/200/300" },
      },
    ])
  ).toEqual("LOOK AT THE KITTEN!\n![](https://placekitten.com/g/200/300)\n");
});

test("encodes image url", function () {
  expect(
    render([
      {
        insert: "LOOK AT THE KITTEN!\n",
      },
      {
        insert: { image: "https://placekitten.com/g/200/300(1).jpg" },
      },
    ])
  ).toEqual(
    "LOOK AT THE KITTEN!\n![](https://placekitten.com/g/200/300%281%29.jpg)\n"
  );
});

test("removes download params for images", function () {
  expect(
    render([
      {
        insert: "LOOK AT THE KITTEN!\n",
      },
      {
        insert: {
          image:
            "https://placekitten.com/g/200/300?params=21312321313&response-content-disposition=attachment; filename=300.jpg",
        },
      },
    ])
  ).toEqual(
    "LOOK AT THE KITTEN!\n![](https://placekitten.com/g/200/300?params=21312321313)\n"
  );
});

test("renders block format", function () {
  expect(
    render([
      {
        insert: "Headline",
      },
      {
        attributes: {
          header: 1,
        },
        insert: "\n",
      },
    ])
  ).toEqual("# Headline\n");
});

test("renders lists with inline formats correctly", function () {
  expect(
    render([
      {
        attributes: {
          italic: true,
        },
        insert: "Glenn v. Brumby",
      },
      {
        insert: ", 663 F.3d 1312 (11th Cir. 2011)",
      },
      {
        attributes: {
          list: "ordered",
        },
        insert: "\n",
      },
      {
        attributes: {
          italic: true,
        },
        insert: "Barnes v. City of Cincinnati",
      },
      {
        insert: ", 401 F.3d 729 (6th Cir. 2005)",
      },
      {
        attributes: {
          list: "ordered",
        },
        insert: "\n",
      },
    ])
  ).toEqual(
    "1. _Glenn v. Brumby_, 663 F.3d 1312 (11th Cir. 2011)\n2. _Barnes v. City of Cincinnati_, 401 F.3d 729 (6th Cir. 2005)\n"
  );
});

test("renders adjacent lists correctly", function () {
  expect(
    render([
      {
        insert: "Item 1",
      },
      {
        insert: "\n",
        attributes: {
          list: "ordered",
        },
      },
      {
        insert: "Item 2",
      },
      {
        insert: "\n",
        attributes: {
          list: "ordered",
        },
      },
      {
        insert: "Item 3",
      },
      {
        insert: "\n",
        attributes: {
          list: "ordered",
        },
      },
      {
        insert: "Intervening paragraph\nItem 4",
      },
      {
        insert: "\n",
        attributes: {
          list: "ordered",
        },
      },
      {
        insert: "Item 5",
      },
      {
        insert: "\n",
        attributes: {
          list: "ordered",
        },
      },
      {
        insert: "Item 6",
      },
      {
        insert: "\n",
        attributes: {
          list: "ordered",
        },
      },
    ])
  ).toEqual(
    "1. Item 1\n2. Item 2\n3. Item 3\n\nIntervening paragraph\n1. Item 4\n2. Item 5\n3. Item 6\n"
  );
});

test("renders adjacent inline formats correctly", function () {
  expect(
    render([
      {
        attributes: {
          italic: true,
        },
        insert: "Italics! ",
      },
      {
        attributes: {
          italic: true,
          link: "http://example.com",
        },
        insert: "Italic link",
      },
      {
        attributes: {
          link: "http://example.com",
        },
        insert: " regular link",
      },
    ])
  ).toEqual(
    "_Italics! [Italic link](http://example.com)_[ regular link](http://example.com)" +
      "\n"
  );
});

test("renders checkboxes correctly", function () {
  expect(
    render([
      {
        insert: "milk",
      },
      {
        attributes: {
          list: "unchecked",
        },
        insert: "\n",
      },
      {
        insert: "cheese",
      },
      {
        attributes: {
          list: "checked",
        },
        insert: "\n",
      },
    ])
  ).toEqual("- [ ] milk\n- [x] cheese\n");
});

test("render an inline link", function () {
  expect(
    render([
      {
        insert: "Go to Google",
        attributes: {
          link: "https://www.google.fr",
        },
      },
    ])
  ).toEqual("[Go to Google](https://www.google.fr)" + "\n");
});

test("renders a separator block", function () {
  expect(
    render([
      {
        insert: "Before\n",
      },
      {
        insert: { thematic_break: true },
      },
      {
        insert: "After\n",
      },
    ])
  ).toEqual("Before" + "\n" + "\n" + "---" + "\n" + "After" + "\n");
});
