type Log = (st: string) => void;

class Logger {
  private debugEnabled: boolean = false;
  private onInfo: Log = () => {};
  private onDebug: Log = () => {};

  constructor() {
    const _global = global /* node */ as any;
    _global.info = (st: string) => this.info(st);
    _global.debug = (st: string) => this.debug(st);
  }

  init(onInfo: Log, onDebug: Log) {
    this.onInfo = onInfo;
    this.onDebug = onDebug;
  }

  set(debug: boolean) {
    this.debugEnabled = debug;
  }

  info(msg: string) {
    this.onInfo(msg);
  }

  debug(msg: string) {
    if (this.debugEnabled) {
      this.onDebug(msg);
    }
  }
}

export const logger = new Logger();
