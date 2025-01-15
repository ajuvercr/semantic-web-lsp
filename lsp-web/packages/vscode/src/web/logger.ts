import * as vscode from "vscode";

const _global = global /* node */ as any;
export const logger = vscode.window.createOutputChannel("Orange"); // This method is called when your extension is activated
_global.logit = (st: string) => logger.appendLine(st);

