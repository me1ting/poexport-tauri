import { invoke } from "@tauri-apps/api/tauri";
import type { Config } from "./types";

// use `Cmd` suffix to avoid name conflicts

export async function getConfigCmd(): Promise<Config> {
    return invoke<Config>("get_config");
}

export async function resetConfigCmd(): Promise<Config> {
    return invoke<Config>("reset_config");
}

export async function setPoeSessionIdCmd(id: string): Promise<null> {
    return invoke<null>("set_poe_session_id", { id });
}

export async function setPobPathCmd(path: string): Promise<null> {
    return invoke<null>("set_pob_path", { path })
}

export async function setPobProxySupportedCmd(supported: boolean): Promise<null> {
    return invoke<null>("set_pob_proxy_supported", { supported })
}