import init, {
  InitOutput,
  serve2,
  ServerConfig,
} from "../../assets/wasm/lsp_web";
import wasmData from "../../assets/wasm/lsp_web_bg.wasm";
import { FromServer, IntoServer } from "common";

let server: null | Server;

function atobPolyfill(base64: string): string {
  // Define the Base64 characters
  const base64Chars =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
  const lookup: { [key: string]: number } = {};

  // Create a lookup table for decoding
  for (let i = 0; i < base64Chars.length; i++) {
    lookup[base64Chars[i]] = i;
  }

  // Remove padding characters and validate input
  const sanitized = base64.replace(/=+$/, "");
  if (sanitized.length % 4 === 1) {
    throw new Error("Invalid Base64 string");
  }

  let binaryString = "";

  // Decode each group of 4 Base64 characters into 3 bytes
  for (let i = 0; i < sanitized.length; i += 4) {
    const chunk =
      (lookup[sanitized[i]] << 18) |
      (lookup[sanitized[i + 1]] << 12) |
      ((lookup[sanitized[i + 2]] || 0) << 6) |
      (lookup[sanitized[i + 3]] || 0);

    binaryString += String.fromCharCode((chunk >> 16) & 0xff);
    if (sanitized[i + 2] !== undefined) {
      binaryString += String.fromCharCode((chunk >> 8) & 0xff);
    }
    if (sanitized[i + 3] !== undefined) {
      binaryString += String.fromCharCode(chunk & 0xff);
    }
  }

  return binaryString;
}

// Example usage:
function decodeBase64(base64: string): Uint8Array {
  const binaryString = atobPolyfill(base64);
  const len = binaryString.length;
  const bytes = new Uint8Array(len);
  for (let i = 0; i < len; i++) {
    bytes[i] = binaryString.charCodeAt(i);
  }
  return bytes;
}

export default class Server {
  readonly initOutput: InitOutput;
  readonly #intoServer: IntoServer;
  readonly #fromServer: FromServer;

  private constructor(
    initOutput: InitOutput,
    intoServer: IntoServer,
    fromServer: FromServer
  ) {
    this.initOutput = initOutput;
    this.#intoServer = intoServer;
    this.#fromServer = fromServer;
  }

  static async initialize(
    intoServer: IntoServer,
    fromServer: FromServer
  ): Promise<Server> {
    if (null == server) {
      const buffer = decodeBase64(
        (<string>(<any>wasmData)).slice("data:application/wasm;base64,".length)
      );
      // const buffer = stringToUint8ArrayDirect(<string>(<any>wasmData));
      // base64 to buffer
      const initOutput = await init(buffer);
      server = new Server(initOutput, intoServer, fromServer);
    } else {
      console.warn("Server already initialized; ignoring");
    }
    return server;
  }

  async start(): Promise<void> {
    const config = new ServerConfig(this.#intoServer, this.#fromServer);
    await serve2(config);
  }
}
