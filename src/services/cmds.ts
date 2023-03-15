import { invoke } from "@tauri-apps/api/tauri";
import type { Config } from "./types";

export async function getConfig(): Promise<Config> {
    return invoke<Config>("get_config");
}
