import * as monaco from "monaco-editor-core";

export type LanguageExtension = monaco.languages.ILanguageExtensionPoint & {
  aliases: string[];
  extensions: string[];
  mimetypes: string[];
};

export const Languages = {
  turtle: {
    id: "turtle",
    extensions: [".ttl"],
    aliases: ["ttl"],
    mimetypes: ["text/turtle"],
  },
  sparql: {
    id: "sparql",
    extensions: [".sq", ".rq"],
    aliases: ["sq", "rq"],
    mimetypes: ["application/sparql-query"],
  },
};
