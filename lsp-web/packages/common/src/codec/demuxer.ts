import * as vsrpc from "vscode-jsonrpc";

import Bytes from "./bytes";
import PromiseMap from "./map";
import Queue from "./queue";

import * as vscode from "vscode";

export default class StreamDemuxer extends Queue<Uint8Array> {
  readonly responses: PromiseMap<number | string, vsrpc.ResponseMessage> =
    new PromiseMap();
  readonly notifications: Queue<vsrpc.NotificationMessage> =
    new Queue<vsrpc.NotificationMessage>();
  readonly requests: Queue<vsrpc.RequestMessage> =
    new Queue<vsrpc.RequestMessage>();

  readonly allMessages: Queue<vsrpc.Message> = new Queue<vsrpc.Message>();

  readonly #start: Promise<void>;

  constructor() {
    super();
    this.#start = this.start();
  }

  private async start(): Promise<void> {
    let contentLength: null | number = null;
    let buffer = new Uint8Array();

    for await (const bytes of this) {
      buffer = Bytes.append(Uint8Array, buffer, bytes);

      // check if the content length is known
      if (null == contentLength) {
        // if not, try to match the prefixed headers
        const match = Bytes.decode(buffer).match(/^Content-Length:\s*(\d+)\s*/);
        if (null == match) continue;

        // try to parse the content-length from the headers
        const length = parseInt(match[1]);
        if (isNaN(length)) throw new Error("invalid content length");

        // slice the headers since we now have the content length
        buffer = buffer.slice(match[0].length);

        // set the content length
        contentLength = length;
      }

      // if the buffer doesn't contain a full message; await another iteration

      while (null !== contentLength && buffer.length >= contentLength) {
        // decode buffer to a string
        const delimited = Bytes.decode(buffer.slice(0, contentLength));

        // reset the buffer
        buffer = buffer.slice(contentLength);

        // reset the contentLength
        contentLength = null;

        const message = JSON.parse(delimited) as vsrpc.Message;

        const match = Bytes.decode(buffer).match(/^Content-Length:\s*(\d+)\s*/);
        if (null != match) {
          const length = parseInt(match[1]);
          if (isNaN(length)) throw new Error("invalid content length");

          // slice the headers since we now have the content length
          buffer = buffer.slice(match[0].length);

          // set the content length
          contentLength = length;
        }

        this.allMessages.enqueue(message);

        // demux the message stream
        if (vsrpc.Message.isResponse(message) && null != message.id) {
          this.responses.set(message.id, message);
          continue;
        }
        if (vsrpc.Message.isNotification(message)) {
          this.notifications.enqueue(message);
          continue;
        }
        if (vsrpc.Message.isRequest(message)) {
          this.requests.enqueue(message);
          continue;
        }
      }
    }
  }
}
