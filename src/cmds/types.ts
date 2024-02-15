export interface Config {
    poeSessId: string;
}

export interface UpdateInfo {
    needUpdate: boolean;
    current: string;
    latest: string;
    changelog: string;
}