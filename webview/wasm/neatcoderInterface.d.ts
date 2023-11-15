export interface Api {
    api_type: number;
    host?: string;
    name: string;
    port?: number;
    schemas: Record<string, string>;
}

export interface AppData {
    interfaces: Record<string, Interface>;
    scaffold: string | undefined;
    specs: string | undefined;
    taskPool: TaskPool;
}

export interface ChatParamsWasm {
}

export interface ChatWasm {
    messages: Array<MessageDataWasm>;
    models: Record<string, ModelDataWasm>;
    sessionId: string;
    title: string;
}

export interface CodeGenParams {
    filename: string;
}

export interface Database {
    db_type: number;
    host?: string;
    name: string;
    port?: number;
    schemas: Record<string, string>;
}

export interface GptMessageWasm {
    content: string;
    role: string;
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
    language: number;
}

export interface MessageDataWasm {
    payload: GptMessageWasm;
    ts: Date;
    user: string;
}

export interface ModelDataWasm {
    id: string;
    interface: string;
    uri: string;
}

export interface Pipeline {
    order: Array<number>;
    tasks: Record<number, Task>;
}

export interface ScaffoldParams {
    specs: string;
}

export interface Storage {
    file_type: number;
    name: string;
    region?: string;
    schemas: Record<string, string>;
    storage_type: number;
}

export interface Task {
    description: string;
    id: number;
    name: string;
    status: number;
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
    counter: number;
}

