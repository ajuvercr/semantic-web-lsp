import * as vscode from "vscode";

export const logger = vscode.window.createOutputChannel("Orange"); // This method is called when your extension is activated

const logItLogger = vscode.window.createOutputChannel("logger"); // This method is called when your extension is activated
const _global = global /* node */ as any;
_global.logit = (st: string) => logItLogger.append(st);
