type LogEntry = {
  id: string;
  log: Log;
  header: HTMLElement;
  contents: HTMLElement;
};

export class Log {
  private contents: HTMLTextAreaElement;
  constructor(contents: HTMLElement) {
    const textArea = document.createElement("textarea");
    textArea.readOnly = true;
    this.contents = textArea;
    contents.append(textArea);
  }

  writeLine(line: string) {
    this.contents.value +=
      new Date().toISOString() + " " + line.trim() + "\n\n";
    this.scroll();
  }

  scroll() {
    this.contents.scrollTop = this.contents.scrollHeight;
  }
}

export class LogHandler {
  private logs: LogEntry[] = [];
  private logsDicts: { [id: string]: LogEntry } = {};

  private headersElement: HTMLDivElement;
  private logsElement: HTMLDivElement;

  constructor() {
    this.headersElement = <HTMLDivElement>(
      document.getElementById("logHeaders")!
    );
    this.logsElement = <HTMLDivElement>document.getElementById("logs")!;
  }

  getByLog(id: string): Log {
    if (this.logsDicts[id]) {
      return this.logsDicts[id].log;
    }

    const header = document.createElement("button");
    header.textContent = id;

    const element = document.createElement("div");
    element.innerHTML = `<p>${id}</p>`;
    element.classList.add("hidden");
    element.classList.add("logElement");

    const entry: LogEntry = {
      header,
      contents: element,
      id: id,
      log: new Log(element),
    };

    header.onclick = () => {
      this.logs.forEach((x) => {
        x.contents.classList.add("hidden");
        x.header.classList.remove("active");
      });
      header.classList.add("active");
      element.classList.remove("hidden");
      entry.log.scroll();
    };

    this.logsElement.append(element);
    const beforeElementIdx = this.logs.findIndex((x) => x.id > id);

    if (beforeElementIdx != -1) {
      this.headersElement.insertBefore(
        header,
        this.logs[beforeElementIdx].header
      );
      this.logs.splice(beforeElementIdx, 0, entry);
    } else {
      this.headersElement.append(header);
      this.logs.push(entry);
    }
    this.logsDicts[id] = entry;

    return entry.log;
  }
}
