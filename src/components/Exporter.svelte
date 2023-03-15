<script lang="ts">
    import CheckCircle from "./icons/CheckCircle24.svelte";
    import Refresh from "./icons/Refresh24.svelte";

    import { PobStatus, SessionStatus } from "../services/types";

    let port = "0000";
    let sessionStatus = SessionStatus.OK;
    let pobStatus = PobStatus.OK;

    let poeAccountName = "";
    let poeAccountNameEncoded = "";

    let isRefreshing = false;

    function refresh(event: MouseEvent) {
        isRefreshing = true;
        setTimeout(() => {
            isRefreshing = false;
        }, 500);
    }

    function patch() {}

    function encode() {}

    function copyEncodedValue() {}
</script>

<div class="container">
    <div class="status">
        <header>
            <h2>状态</h2>
            <span
                class="icon pointer refresh"
                class:refresh-start={isRefreshing}
                on:click={refresh}
                title="刷新"><Refresh /></span
            >
        </header>
        <div class="line">
            <span class="line-content">
                <span class="line-left">POESESSID</span>
                <span class="line-right">
                    {#if sessionStatus === SessionStatus.OK}
                        <span class="icon ok"><CheckCircle /></span>
                    {:else}
                        <span class="icon warning">error</span>
                    {/if}
                </span>
            </span>
        </div>
        <div class="line">
            <span class="line-content">
                <span class="line-left">POB</span>
                <span class="line-right">
                    {#if pobStatus === PobStatus.OK}
                        <span class="icon ok"><CheckCircle /></span>
                    {:else if pobStatus === PobStatus.NEEDPATCH}
                        <span class="update pointer" on:click={patch}>更新</span
                        >
                    {:else}
                        <span class="icon warning">error</span>
                    {/if}
                </span>
            </span>
        </div>
        <div class="line">
            <span class="line-content">
                <span class="line-left">监听端口</span>
                <span class="line-right">{port}</span>
            </span>
        </div>
    </div>
    <div class="encoder">
        <h2>URL 编码</h2>
        <div class="line">
            <span class="line-content">
                <input placeholder="论坛账户名" bind:value={poeAccountName} />
                <button on:click={encode} disabled={poeAccountName === ""}
                    >编码</button
                >
            </span>
        </div>
        <div class="line">
            <span class="line-content">
                <input
                    placeholder="编码结果"
                    bind:value={poeAccountNameEncoded}
                    disabled
                />
                <button
                    disabled={poeAccountNameEncoded === ""}
                    on:click={copyEncodedValue}>复制</button
                >
            </span>
        </div>
    </div>
</div>

<style>
    .container {
        width: 420px;
        margin: 0 auto;
    }

    h2 {
        margin-left: 10px;
    }

    .container > div:nth-child(n + 2) {
        margin-top: 30px;
        border-top: 1px solid #dddddd;
    }

    .line {
        font-size: 16px;
        line-height: 28px;
    }

    .line-content {
        padding: 0 10px;
        display: flex;
    }

    .status > header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .status > header > .refresh {
        margin-right: 10px;
    }

    @keyframes rotate {
        from {
            transform: rotate(0deg);
        }

        to {
            transform: rotate(360deg);
        }
    }

    .refresh-start {
        animation-name: rotate;
        animation-duration: 1s;
        animation-iteration-count: infinite;
        animation-timing-function: linear;
        animation-play-state: running;
    }

    .status .line:hover {
        background-color: #f5f5f5;
    }

    .status .line-content {
        justify-content: space-between;
        align-items: center;
    }

    .update {
        vertical-align: top;
        border-bottom: 1px dashed;
        color: red;
    }

    .encoder .line {
        margin: 5px 0;
    }

    .encoder button {
        margin-left: 5px;
    }
</style>
