// import init, { InitOutput, serve, ServerConfig } from "../assets/wasm/lsp_web";
import init, { InitOutput, serve, ServerConfig } from "../assets/wasm/lsp_web";
import { FromServer, IntoServer } from "common";

let server: null | Server;

export default class Server {
  readonly initOutput: InitOutput;
  readonly #intoServer: IntoServer;
  readonly #fromServer: FromServer;

  private constructor(initOutput: InitOutput, intoServer: IntoServer, fromServer: FromServer) {
    this.initOutput = initOutput;
    this.#intoServer = intoServer;
    this.#fromServer = fromServer;
  }

  static async initialize(intoServer: IntoServer, fromServer: FromServer): Promise<Server> {
    if (null == server) {
      const initOutput = await init();
      server = new Server(initOutput, intoServer, fromServer);
    } else {
      console.warn("Server already initialized; ignoring");
    }
    return server;
  }

  async start(): Promise<void> {
    const config = new ServerConfig(this.#intoServer, this.#fromServer);
    await serve(config);
  }
}
