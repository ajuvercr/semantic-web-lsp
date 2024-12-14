import * as jsrpc from "json-rpc-2.0";
import { JSONRPCRequest } from "json-rpc-2.0";
import {
  AbstractMessageReader,
  AbstractMessageWriter,
  DataCallback,
  Disposable,
  MessageWriter,
} from "vscode-jsonrpc";
import * as proto from "vscode-languageserver-protocol";
import { MessageReader, RequestMessage } from "vscode-languageserver-protocol";

import { Codec, FromServer, IntoServer } from "./codec";

const consoleChannel = document.getElementById(
  "channel-console"
) as HTMLTextAreaElement;

class Reader extends AbstractMessageReader {
  private callBacks: DataCallback[] = [];
  private fromServer: FromServer;

  constructor(fromServer: FromServer) {
    super();
    this.fromServer = fromServer;
  }

  async init(): Promise<void> {
    for await (const request of this.fromServer.requests) {
      for (let cb of this.callBacks) {
        cb(request);
      }
    }
  }

  listen(callback: DataCallback): Disposable {
    this.callBacks.push(callback);
    return {
      dispose() {},
    };
  }
}

class Writer extends AbstractMessageWriter {
  intoServer: IntoServer;
  fromServer: FromServer;
  constructor(intoServer: IntoServer, fromServer: FromServer) {
    super();
    this.intoServer = intoServer;
    this.fromServer = fromServer;
  }

  async write(msg: RequestMessage) {
    const encoded = Codec.encode(<JSONRPCRequest>msg);
    this.intoServer.enqueue(encoded);

    if (null != msg.id) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const response = await this.fromServer.responses.get(msg.id)!;
      console.log(response);
    }
  }

  end(): void {}
}

export class NewClient {
  #fromServer: FromServer;
  #intoServer: IntoServer;
  constructor(fromServer: FromServer, intoServer: IntoServer) {
    this.#intoServer = intoServer;
    this.#fromServer = fromServer;
  }

  reader(): MessageReader {
    return new Reader(this.#fromServer);
  }

  writer(): MessageWriter {
    return new Writer(this.#intoServer, this.#fromServer);
  }
}

export default class Client extends jsrpc.JSONRPCServerAndClient {
  afterInitializedHooks: (() => Promise<void>)[] = [];
  #fromServer: FromServer;

  onLegend: (legen: any) => void = () => {};

  constructor(fromServer: FromServer, intoServer: IntoServer) {
    super(
      new jsrpc.JSONRPCServer(),
      new jsrpc.JSONRPCClient(async (json: jsrpc.JSONRPCRequest) => {
        const encoded = Codec.encode(json);
        intoServer.enqueue(encoded);
        if (null != json.id) {
          // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
          const response = await fromServer.responses.get(json.id)!;
          this.client.receive(response as jsrpc.JSONRPCResponse);
        }
      })
    );
    this.#fromServer = fromServer;
  }

  async start(onDiagnostic: (diags: proto.PublishDiagnosticsParams) => void): Promise<void> {
    // process "window/logMessage": client <- server
    this.addMethod(proto.LogMessageNotification.type.method, (params) => {
      const { type, message } = params as {
        type: proto.MessageType;
        message: string;
      };
      switch (type) {
        case proto.MessageType.Error: {
          consoleChannel.value += "[error] ";
          break;
        }
        case proto.MessageType.Warning: {
          consoleChannel.value += " [warn] ";
          break;
        }
        case proto.MessageType.Info: {
          consoleChannel.value += " [info] ";
          break;
        }
        case proto.MessageType.Log: {
          consoleChannel.value += "  [log] ";
          break;
        }
      }
      consoleChannel.value += message;
      consoleChannel.value += "\n";
      return;
    });

    this.addMethod(proto.PublishDiagnosticsNotification.type.method, (params) => {
      onDiagnostic(params);
    });

    // request "initialize": client <-> server
    const resp: any = await (this.request(proto.InitializeRequest.type.method, {
      processId: null,
      clientInfo: {
        name: "demo-language-client",
      },
      capabilities: {
        textDocument: {
          publishDiagnostics: {},
        },
      },
      rootUri: null,
    } as proto.InitializeParams) as Promise<jsrpc.JSONRPCResponse>);

    this.onLegend(resp.capabilities.semanticTokensProvider.legend);

    // notify "initialized": client --> server
    this.notify(proto.InitializedNotification.type.method, {});

    await Promise.all(
      this.afterInitializedHooks.map((f: () => Promise<void>) => f())
    );
    await Promise.all([this.processNotifications(), this.processRequests()]);
  }

  async processNotifications(): Promise<void> {
    for await (const notification of this.#fromServer.notifications) {
      await this.receiveAndSend(notification);
    }
  }

  async processRequests(): Promise<void> {
    for await (const request of this.#fromServer.requests) {
      await this.receiveAndSend(request);
    }
  }

  pushAfterInitializeHook(...hooks: (() => Promise<void>)[]): void {
    this.afterInitializedHooks.push(...hooks);
  }
}
