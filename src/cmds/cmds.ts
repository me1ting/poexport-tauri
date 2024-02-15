import { invoke } from "@tauri-apps/api/tauri";
import { Config, UpdateInfo } from "./types";

// use `Cmd` suffix to avoid name conflicts

export async function getConfigCmd(): Promise<Config> {
    return invoke<Config>("get_config");
}

export async function setPoeSessIdCmd(poeSessId: string): Promise<void> {
    return invoke<void>("set_poesessid", { poesessid: poeSessId });
}

export async function getCharactersCmd(accountName: string): Promise<string> {
    return invoke<string>("get_characters", { accountName });
}

export async function getItemsCmd(accountName: string, character: string): Promise<string> {
    return invoke<string>("get_items", { accountName,character });
}

export async function getPassiveSkillsCmd(accountName: string, character: string): Promise<string> {
    return invoke<string>("get_passive_skills", { accountName,character });
}


export async function checkUpdateCmd(): Promise<UpdateInfo> {
    return invoke<UpdateInfo>("check_for_update");
}