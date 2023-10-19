export interface Api {
schemas: Record<string, string>;
}
export interface AppState {
interfaces: Record<string, Interface>;
scaffold: string | undefined;
specs: string | undefined;
taskPool: TaskPool;
}
export interface Chat {
sessionId: string;
title: string;
}
export interface Chats {
}
export interface CodeGenParams {
filename: string;
}
export interface Database {
schemas: Record<string, string>;
}
export interface Interface {
interface: any;
interfaceType: number;
itype: string;
name: string;
schemas: Record<string, string>;
}
export interface InterfaceInner {
api: Api | undefined;
database: Database | undefined;
storage: Storage | undefined;
}
export interface Language {
}
export interface Message {
}
export interface Model {
}
export interface OpenAIMsg {
}
export interface OpenAIParams {
}
export interface Pipeline {
order: Array<number>;
tasks: Record<number, Task>;
}
export interface ScaffoldParams {
specs: string;
}
export interface Storage {
schemas: Record<string, string>;
}
export interface Task {
description: string;
name: string;
taskParams: TaskParams;
}
export interface TaskParams {
inner: TaskParamsInner;
scaffoldProject: ScaffoldParams | undefined;
streamCode: CodeGenParams | undefined;
taskType: number;
}
export interface TaskParamsInner {
scaffoldProject: ScaffoldParams | undefined;
streamCode: CodeGenParams | undefined;
}
export interface TaskPool {
}
