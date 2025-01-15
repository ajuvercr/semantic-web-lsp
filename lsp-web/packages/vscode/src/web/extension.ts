// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from "vscode";
import Server from "./server";
import { Codec, FromServer, IntoServer, jsrpc } from "common";
import { logger } from "./logger";
import {
  BaseLanguageClient,
  ConnectionOptions,
  Disposable,
  LanguageClientOptions,
  MessageTransports,
} from "vscode-languageclient";
import * as rpc from "vscode-jsonrpc";
// import { LanguageClient } from "vscode-languageclient/browser";
//
class ReadableS {
  readonly onError: rpc.Event<Error>;
  readonly onClose: rpc.Event<void>;
  readonly onPartialMessage: rpc.Event<rpc.PartialMessageInfo>;
  readonly emitter: rpc.Emitter<rpc.Message>;

  constructor(fromServer: FromServer) {
    const emitter = new rpc.Emitter<rpc.Message>();
    this.emitter = emitter;

    const errorEmitter = new rpc.Emitter<Error>();
    this.onError = errorEmitter.event;

    const closeEmitter = new rpc.Emitter<void>();
    this.onClose = closeEmitter.event;

    const partialEmitter = new rpc.Emitter<rpc.PartialMessageInfo>();
    this.onPartialMessage = partialEmitter.event;

    (async () => {
      for await (const line of fromServer.allMessages) {
        logger.appendLine("Got string from server " + JSON.stringify(line));
        emitter.fire(line);
      }
    })();
  }
  // /**
  //  * Begins listening for incoming messages. To be called at most once.
  //  * @param callback A callback for receiving decoded messages.
  //  */
  listen(callback: rpc.DataCallback): Disposable {
    return this.emitter.event(callback);
  }
  // /** Releases resources incurred from reading or raising events. Does NOT close the underlying transport, if any. */
  dispose(): void {}
}

class WriterS {
  readonly onError: rpc.Event<
    [Error, rpc.Message | undefined, number | undefined]
  >;
  /**
   * An event raised when the underlying transport has closed and writing is no longer possible.
   */
  readonly onClose: rpc.Event<void>;
  readonly intoServer: IntoServer;

  constructor(intoServer: IntoServer) {
    const errorEmitter = new rpc.Emitter<
      [Error, rpc.Message | undefined, number | undefined]
    >();
    this.onError = errorEmitter.event;

    const closeEmitter = new rpc.Emitter<void>();
    this.onClose = closeEmitter.event;
    this.intoServer = intoServer;
  }
  /**
   * Sends a JSON-RPC message.
   * @param msg The JSON-RPC message to be sent.
   * @description Implementations should guarantee messages are transmitted in the same order that they are received by this method.
   */
  async write(msg: rpc.Message): Promise<void> {
    logger.appendLine("Writing line to server " + JSON.stringify(msg));
    const encoded = Codec.encode(<jsrpc.JSONRPCRequest>msg);
    this.intoServer.enqueue(encoded);
  }
  /**
   * Call when the connection using this message writer ends
   * (e.g. MessageConnection.end() is called)
   */
  end(): void {}
  /** Releases resources incurred from writing or raising events. Does NOT close the underlying transport, if any. */
  dispose(): void {}
}

class LanguageClient extends BaseLanguageClient {
  private readonly reader: FromServer;
  private readonly writer: IntoServer;

  constructor(
    id: string,
    name: string,
    clientOptions: LanguageClientOptions,
    reader: FromServer,
    writer: IntoServer
  ) {
    super(id, name, clientOptions);
    this.reader = reader;
    this.writer = writer;
  }

  protected async createMessageTransports(
    _encoding: string
  ): Promise<MessageTransports> {
    return {
      reader: new ReadableS(this.reader),
      writer: new WriterS(this.writer),
    };
  }
}

// Your extension is activated the very first time the command is executed
export async function activate(context: vscode.ExtensionContext) {
  logger.appendLine("semantic-web-lsp activated!, Part 3");
  // Use the console to output diagnostic information (console.log) and errors (console.error)
  // This line of code will only be executed once when your extension is activated
  console.log(
    'Congratulations, your extension "semantic-web-lsp" is now active in the web extension host!'
  );

  const intoServer = new IntoServer();
  const fromServer = FromServer.create();
  logger.appendLine("Created intoServer and fromServer");

  const serverPromise = Server.initialize(intoServer, fromServer);
  logger.appendLine("Building a server");
  const server = await serverPromise;
  logger.appendLine("Server built");

  // Options to control the language client
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ language: "turtle" }],
    synchronize: {},
    initializationOptions: {},
  };

  const client = new LanguageClient(
    "semantic-web-lsp",
    "semantic-web-lsp",
    clientOptions,
    fromServer,
    intoServer
  );

  logger.appendLine("Here1");
  server.start();
  logger.appendLine("Here2");

  await new Promise((res) => setTimeout(res, 200));
  await client.start();
  logger.appendLine("Here3");
}

// This method is called when your extension is deactivated
export function deactivate() {}
