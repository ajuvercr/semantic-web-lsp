import debounce from "debounce";
import * as monaco from "monaco-editor-core";
import { MonacoToProtocolConverter } from "monaco-languageclient";
import * as proto from "vscode-languageserver-protocol";

import Client from "./client";
import { FromServer, IntoServer } from "common";
import { Languages } from "./language";
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

    readonly editors: monaco.editor.IEditor[] = [];
    initializeMonaco(): void {
        this.#window.MonacoEnvironment = new Environment();
    }

    addEditor(
        client: Client,
        init: ModelStart,
        languageId: string
    ): monaco.editor.ITextModel {
        const model = monaco.editor.createModel(
            init.value,
            languageId,
            monaco.Uri.parse(init.url)
        );

        client.editors[init.url] = model;

        const change = debounce(() => {
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
        }, 50);

        const save = debounce(() => {
            client.notify(proto.DidSaveTextDocumentNotification.type.method, {
                textDocument: {
                    version: 0,
                    uri: model.uri.toString(),
                },
            } as proto.DidChangeTextDocumentParams);
        }, 2000);
        model.onDidChangeContent(() => {
            change();
            save();
        });

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

            save();
        });

        const container = document.getElementById(init.elementId)!; // eslint-disable-line @typescript-eslint/no-non-null-assertion
        const editor = monaco.editor.create(container, {
            model,
            automaticLayout: true,
            "semanticHighlighting.enabled": true,
            minimap: {
                enabled: false,
            },
            quickSuggestions: false,
            scrollBeyondLastLine: false,
            links: false,
        });

        editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyK, () => {
            const focusedEditor = this.editors.find((e) => e.hasTextFocus());
            if (focusedEditor) {
                focusedEditor.trigger("keyboard", "editor.action.rename", null);
            }
        });

        this.editors.push(editor);

        return model;
    }

    async run(): Promise<void> {
        const client = new Client(this.#fromServer, this.#intoServer);
        const server = await Server.initialize(this.#intoServer, this.#fromServer);
        const turtleId = client.addLanguage(Languages.turtle);
        const sparqlId = client.addLanguage(Languages.sparql);

        this.initializeMonaco();

        this.addEditor(client, editors.sparql, sparqlId);
        this.addEditor(client, editors.turtle, turtleId);
        this.addEditor(client, editors.owl, turtleId);
        this.addEditor(client, editors.shacl, turtleId);

        await Promise.all([server.start(), client.start()]);
    }
}

type Keys = "turtle" | "sparql" | "owl" | "shacl";
const editors: { [K in Keys]: ModelStart } = {
    turtle: {
        value: `@prefix owl: <http://www.w3.org/2002/07/owl#>.
@prefix ex: <http://example.org/>.
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#>.
@prefix xsd: <http://www.w3.org/2001/XMLSchema#>.
@prefix ed: <./owl.ttl#>.

<> owl:imports <./shacl.ttl#>.

<HoverFeature> a ed:Hover.
<CompleteFeature> a ed:Completion;
  ed:completesTypes "true"^^xsd:boolean;
  ed:completesProperties "true"^^xsd:boolean;
  ed:isCool "true".

<SWLS> a ed:LanguageServer;
  rdfs:label "test1", "test2";
  ed:hasFeature <HoverFeature>, <CompleteFeature>.
`,
        url: "inmemory://examples.this/turtle.ttl",
        elementId: "editor",
    },
    sparql: {
        value: `PREFIX  ed: <./owl.ttl#>

SELECT  *
{
  ?s a ed:LanguageServer;
    ed:hasFeature ?feature.
  
  ?feature a ?featureType.
  OPTIONAL {
    ?feature ed:isCool ?isCool.
  }
}
`,
        url: "inmemory://examples.this/query.sq",
        elementId: "editor2",
    },
    owl: {
        value: `@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>.
@prefix xsd: <http://www.w3.org/2001/XMLSchema#>.
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#>.
@prefix owl: <http://www.w3.org/2002/07/owl#>.
@prefix : <#>.

:LanguageServer a rdfs:Class;
  rdfs:label "Language Server";
  rdfs:comment "LSP with supported features".

:hasFeature a rdf:Property;
  rdfs:label "Has Feature";
  rdfs:comment "Links a property to a language server";
  rdfs:domain :LanguageServer;
  rdfs:range :Feature.

:Feature a rdfs:Class;
  rdfs:label "Feature of a Language Server";
  rdfs:comment "A feature supported by a LSP".

:Hover a rdfs:Class;
  rdfs:subClassOf :Feature;
  rdfs:label "Hover Feature";
  rdfs:comment "The LSP supports the hover action".

:Completion a rdfs:Class;
  rdfs:subClassOf :Feature;
  rdfs:label "Completion Feature";
  rdfs:comment "The LSP can autocomplete".

:completesTypes a rdf:Property;
  rdfs:label "Completes Types";
  rdfs:comment "Indicates that the hover action can complete types";
  rdfs:domain :Completion;
  rdfs:range xsd:boolean.

:completesProperties a rdf:Property;
  rdfs:label "Completes Properties";
  rdfs:comment "Indicates that the hover action can complete properties";
  rdfs:domain :Completion;
  rdfs:range xsd:boolean.

:isCool a rdf:Property;
  rdfs:label "Cool Feature";
  rdfs:comment "Indicates whether or not a feature is cool";
  rdfs:domain :Feature;
  rdfs:range xsd:boolean.
`,
        url: "inmemory://examples.this/owl.ttl",
        elementId: "editor3",
    },
    shacl: {
        value: `@prefix ex: <http://example.org/>.
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix ed: <./owl.ttl#>.

ed:LanguageFeatureShape 
    a sh:NodeShape;
    sh:targetClass ed:Feature, ed:Hover, ed:Completion;
    sh:property [
        sh:path ed:isCool;
        sh:datatype xsd:boolean;
        sh:name "Coolness";
        sh:minCount 1;
        sh:in ( "true" );
    ].
    
ed:LanguageServerShape
    a sh:NodeShape;
    sh:targetClass ed:LanguageServer;
    sh:property [
        sh:path ed:hasFeature;
        sh:minCount 3;
        sh:node ed:LanguageFeatureShape;
    ].
`,
        url: "inmemory://examples.this/shacl.ttl",
        elementId: "editor4",
    },
};
