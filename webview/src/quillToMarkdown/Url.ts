const encodeLink = (link: string): string =>
  encodeURI(link)
    .replace(/\(/i, "%28")
    .replace(/\)/i, "%29")
    .replace(/(\?|&)response-content-disposition=attachment.*$/, "");

export {
  encodeLink,
};
