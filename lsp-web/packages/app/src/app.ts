import debounce from "debounce";
import * as monaco from "monaco-editor-core";
import { MonacoToProtocolConverter } from "monaco-languageclient";
import * as proto from "vscode-languageserver-protocol";

import Client from "./client";
import { FromServer, IntoServer } from "./codec";
import Language, { Languages, protocolToMonaco } from "./language";
import Server from "./server";

class Environment implements monaco.Environment {
  getWorkerUrl(moduleId: string, label: string) {
    if (label === "editorWorkerService") {
      return "./editor.worker.bundle.js";
    }
    throw new Error(
      `getWorkerUrl: unexpected ${JSON.stringify({ moduleId, label })}`
    );
  }
}

const monacoToProtocol = new MonacoToProtocolConverter(monaco);

type ModelStart = {
  value: string;
  url: string;
  elementId: string;
};

export default class App {
  readonly #window: Window & monaco.Window & typeof globalThis = self;

  readonly #intoServer: IntoServer = new IntoServer();
  readonly #fromServer: FromServer = FromServer.create();

  initializeMonaco(): void {
    this.#window.MonacoEnvironment = new Environment();
  }

  addEditor(
    client: Client,
    init: ModelStart,
    languageId: string
  ): monaco.editor.ITextModel {
    // const value = `
    // @prefix rml: <http://w3id.org/rml/core#>.
    // @prefix tree: <https://w3id.org/tree#>.
    // @prefix foaf: <http://xmlns.com/foaf/0.1/>.
    //
    //
    // [ ] a foaf:Project;
    //   foaf:name "Arthur", "Testing";.
    //
    // <a> a foaf:Person;
    //   foaf:name "ben"^^xsd:string;
    //   foaf:nick "Benny".
    // `.replace(/^\s*\n/gm, "");
    // const uri = monaco.Uri.parse("inmemory://demo.ttl");

    const model = monaco.editor.createModel(
      init.value,
      languageId,
      monaco.Uri.parse(init.url)
    );

    client.editors[init.url] = model;

    model.onDidChangeContent(
      debounce(() => {
        const text = model.getValue();
        client.notify(proto.DidChangeTextDocumentNotification.type.method, {
          textDocument: {
            version: 0,
            uri: model.uri.toString(),
          },
          contentChanges: [
            {
              range: monacoToProtocol.asRange(model.getFullModelRange()),
              text,
            },
          ],
        } as proto.DidChangeTextDocumentParams);
      }, 200)
    );

    // eslint-disable-next-line @typescript-eslint/require-await
    client.pushAfterInitializeHook(async () => {
      client.notify(proto.DidOpenTextDocumentNotification.type.method, {
        textDocument: {
          uri: model.uri.toString(),
          languageId: languageId,
          version: 0,
          text: model.getValue(),
        },
      } as proto.DidOpenTextDocumentParams);
    });

    const container = document.getElementById(init.elementId)!; // eslint-disable-line @typescript-eslint/no-non-null-assertion
    monaco.editor.create(container, {
      model,
      automaticLayout: true,
      "semanticHighlighting.enabled": true,
    });

    return model;
  }

  // createEditor(client: Client): monaco.editor.ITextModel {
  //   const container = document.getElementById("editor")!; // eslint-disable-line @typescript-eslint/no-non-null-assertion
  //   const model = this.addEditor(client);
  //   monaco.editor.create(container, {
  //     model,
  //     automaticLayout: true,
  //     "semanticHighlighting.enabled": true,
  //   });
  //   return model;
  // }

  async run(): Promise<void> {
    const client = new Client(this.#fromServer, this.#intoServer);
    const server = await Server.initialize(this.#intoServer, this.#fromServer);
    const turtleId = client.addLanguage(Languages.turtle);
    const sparqlId = client.addLanguage(Languages.sparql);

    this.initializeMonaco();

    this.addEditor(client, editors.turtle, turtleId);
    this.addEditor(client, editors.owl, turtleId);
    this.addEditor(client, editors.shacl, turtleId);
    this.addEditor(client, editors.sparql, sparqlId);

    await Promise.all([server.start(), client.start()]);
  }
}

type Keys = "turtle" | "sparql" | "owl" | "shacl";
const editors: { [K in Keys]: ModelStart } = {
  turtle: {
    value: `
@prefix rml: <http://w3id.org/rml/core#>.
@prefix tree: <https://w3id.org/tree#>.
@prefix foaf: <http://xmlns.com/foaf/0.1/>.
@prefix ed: <./owl.ttl#>.


[ ] a foaf:Project;
  foaf:name "Arthur", "Testing";.

<a> a foaf:Person;
  foaf:name "ben"^^xsd:string;
  foaf:nick "Benny".
    `,
    url: "inmemory://examples.this/demo.ttl",
    elementId: "editor",
  },
  sparql: {
    value: `
PREFIX tree: <https://w3id.org/tree#>
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
PREFIX  dc:  <http://purl.org/dc/elements/1.1/>
PREFIX  ns:  <http://example.org/ns#>

SELECT  *
{
   [] a foaf:Image;
    a foaf:Person,.

   ?person a foaf:Person;
     rdfs:subClassOf ?name, ?name.
}
    `,
    url: "inmemory://examples.this/query.sq",
    elementId: "editor2",
  },
  owl: {
    value: `@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#>.
@prefix owl: <http://www.w3.org/2002/07/owl#>.
@prefix rml: <http://w3id.org/rml/core#>.
@prefix tree: <https://w3id.org/tree#>.
@prefix foaf: <http://xmlns.com/foaf/0.1/>.
@prefix : <#>.

:Test a owl:Class, rdfs:Class;
	rdfs:label "Person" ;
	rdfs:comment "A person." ;
  .

[ ] a foaf:Project;
  foaf:name "Arthur", "Testing";.

<a> a foaf:Person;
  foaf:name "ben"^^xsd:string;
  foaf:nick "Benny".
    
    `,
    url: "inmemory://examples.this/owl.ttl",
    elementId: "editor3",
  },
  shacl: {
    value: `
@prefix rml: <http://w3id.org/rml/core#>.
@prefix tree: <https://w3id.org/tree#>.
@prefix foaf: <http://xmlns.com/foaf/0.1/>.

[ ] a foaf:Project;
  foaf:name "Arthur", "Testing";.

<a> a foaf:Person;
  foaf:name "ben"^^xsd:string;
  foaf:nick "Benny".
    `,
    url: "inmemory://examples.this/shacl.ttl",
    elementId: "editor4",
  },
};
