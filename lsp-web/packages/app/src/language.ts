// import * as jsrpc from "json-rpc-2.0";
import {
  MonacoToProtocolConverter,
  ProtocolToMonacoConverter,
} from "monaco-languageclient";
import * as monaco from "monaco-editor-core";
import * as proto from "vscode-languageserver-protocol";

import Client from "./client";

export const monacoToProtocol = new MonacoToProtocolConverter(monaco);
export const protocolToMonaco = new ProtocolToMonacoConverter(monaco);

let language: null | Language;

export default class Language
  implements monaco.languages.ILanguageExtensionPoint
{
  readonly id: string;
  readonly aliases: string[];
  readonly extensions: string[];
  readonly mimetypes: string[];

  private constructor(client: Client) {
    const { id, aliases, extensions, mimetypes } = Language.extensionPoint();
    this.id = id;
    this.aliases = aliases;
    this.extensions = extensions;
    this.mimetypes = mimetypes;
    this.registerLanguage(client);
  }

  static extensionPoint(): monaco.languages.ILanguageExtensionPoint & {
    aliases: string[];
    extensions: string[];
    mimetypes: string[];
  } {
    const id = "turtle";
    const aliases = ["ttl"];
    const extensions = [".ttl"];
    const mimetypes = ["text/turtle"];
    return { id, extensions, aliases, mimetypes };
  }

  private registerLanguage(client: Client): void {
    void client;
    monaco.languages.register(Language.extensionPoint());
    monaco.languages.registerDocumentSymbolProvider(this.id, {
      // eslint-disable-next-line
      async provideDocumentSymbols(
        model,
        token
      ): Promise<monaco.languages.DocumentSymbol[]> {
        void token;
        const response = await (client.request(
          proto.DocumentSymbolRequest.type.method,
          {
            textDocument: monacoToProtocol.asTextDocumentIdentifier(model),
          } as proto.DocumentSymbolParams
        ) as Promise<proto.SymbolInformation[]>);

        const uri = model.uri.toString();

        // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
        const result: monaco.languages.DocumentSymbol[] =
          protocolToMonaco.asSymbolInformations(response, uri);

        return result;
      },
    });
    monaco.languages.registerCompletionItemProvider(this.id, {
      async provideCompletionItems(model, position, _token, _context) {
        console.log("Completion item", model, position);

        console.log({
          textDocument: monacoToProtocol.asTextDocumentIdentifier(model),
          position: monacoToProtocol.asPosition(
            position.lineNumber,
            position.column
          ),
        } as proto.CompletionParams);
        const response = await client.request(
          proto.CompletionRequest.type.method,
          {
            textDocument: monacoToProtocol.asTextDocumentIdentifier(model),
            position: monacoToProtocol.asPosition(
              position.lineNumber,
              position.column
            ),
          } as proto.CompletionParams
        );
        console.log("response", response);
        let out = {
          incomplete: false,
          suggestions: [],
        };

        try {
          out = protocolToMonaco.asCompletionResult({isIncomplete: false, items: response}, {
            startLineNumber: 1,
            startColumn: 1,
            endLineNumber: 1,
            endColumn: 1,
          });
        } catch (ex: any) {
          console.log(ex);
        }
        console.log(out);

        return out;
      },
      // async resolveCompletionItem(item, _token) {
      //   return await jsonService
      //     .doResolve(codeConverter.asCompletionItem(item))
      //     .then((result) => protocolConverter.asCompletionItem(result));
      // },
    });
  }

  static initialize(client: Client): Language {
    if (null == language) {
      language = new Language(client);
    } else {
      console.warn("Language already initialized; ignoring");
    }
    return language;
  }
}
