<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import { notifyError } from "../utils/notify";
    import {
        getConfigCmd,
        resetConfigCmd,
        setPobPathCmd,
        setPobProxySupportedCmd,
        setPoeSessionIdCmd,
    } from "../services/cmds";

    let poeSessionId = "";
    let pobPath = "";
    let pobProxySupported = false;

    async function loadConfig() {
        let config = await getConfigCmd();
        poeSessionId = config.poeSessionId;
        pobPath = config.pobPath;
        pobProxySupported = config.pobProxySupported;
    }

    async function resetConfig() {
        try {
            let config = await resetConfigCmd();
            poeSessionId = config.poeSessionId;
            pobPath = config.pobPath;
            pobProxySupported = config.pobProxySupported;
        } catch (err) {
            notifyError(err);
        }
    }

    async function setPoeSessionId() {
        try {
            await setPoeSessionIdCmd(poeSessionId);
        } catch (err) {
            notifyError(err);
        }
    }

    async function setPobPath() {
        const selected = await open({
            directory: true,
            multiple: false,
            defaultPath: pobPath,
        });
        if (selected !== null && selected !== pobPath) {
            let path: string;
            if (Array.isArray(selected)) {
                path = selected[0];
            } else {
                path = selected;
            }

            try {
                await setPobPathCmd(path);
                pobPath = path;
            } catch (err) {
                notifyError(err);
            }
        }
    }

    function resetPob() {}

    async function switchPobProxySupported() {
        try {
            await setPobProxySupportedCmd(pobProxySupported);
        } catch (err) {
            notifyError(err);
        }
    }

    loadConfig();
</script>

<div class="container">
    <div class="header-container">
        <header class="header">
            <h1>设置</h1>
            <div>
                <div class="button pointer" on:click={resetConfig}>重置</div>
            </div>
        </header>
    </div>
    <div class="details-container">
        <div class="details">
            <div class="details-section">
                <header>
                    <h2>基本</h2>
                </header>
                <div class="section-lines">
                    <div class="line">
                        <label for="poeSessionId">POESESSID</label>
                        <div class="line-right">
                            <input
                                name="poeSessionId"
                                bind:value={poeSessionId}
                            />
                            <button class="pointer" on:click={setPoeSessionId}
                                >保存</button
                            >
                        </div>
                    </div>
                    <div class="line">
                        <label for="pobPath">POB文件夹</label>
                        <div class="line-right">
                            <input
                                name="pobPath"
                                bind:value={pobPath}
                                disabled
                            />
                            <button class="pointer" on:click={setPobPath}
                                >选择</button
                            >
                        </div>
                    </div>
                </div>
            </div>
            <div class="details-section">
                <header>
                    <h2>Patch</h2>
                </header>
                <div class="section-lines">
                    <div class="line">
                        <div class="line-left">
                            <label for="poeSessionId">清除Patch</label>
                        </div>
                        <div class="line-right">
                            <button class="pointer" on:click={resetPob}
                                >重置</button
                            >
                        </div>
                    </div>
                    <div class="line">
                        <div class="line-left">
                            <label for="poeSessionId">Proxy支持</label>
                        </div>
                        <div class="line-right">
                            <input
                                type="checkbox"
                                class="pointer"
                                bind:checked={pobProxySupported}
                                on:change={switchPobProxySupported}
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    .header-container {
        border-bottom: 1px solid #dddddd;
    }

    .header {
        margin: 0 20px;
        display: flex;
        align-items: baseline;
        justify-content: space-between;
    }

    .header .button {
        background-color: #f0f0f0;
        border-radius: 3px;
        padding: 4px 11px;
        vertical-align: middle;
        color: red;
    }

    .details-container {
        margin: 20px;
    }

    .details-section {
        margin-bottom: 10px;
    }

    .section-lines {
        padding: 3px;
        background-color: #eeeeee;
        border-radius: 3px;
    }

    .section-lines > .line {
        display: flex;
        justify-content: space-between;
        padding: 5px 10px;
        border-radius: 3px;
    }

    .section-lines > .line:hover {
        background-color: #cccccc;
    }

    .section-lines > .line > .line-left {
        display: flex;
    }

    .section-lines > .line > .line-right {
        vertical-align: bottom;
    }

    .section-lines > .line > .line-right > button {
        display: inline;
        margin-left: 10px;
    }
</style>
