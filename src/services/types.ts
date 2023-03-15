export interface Config {
    poeSessionId: string;
    pobPath: string;
    listeningPort: number;
    pobProxySupported: boolean;
}

export enum SessionStatus {
    OK,
    INVALID,
}

export enum PobStatus {
    OK,
    NEEDPATCH,
    NOTFOUND,
}

