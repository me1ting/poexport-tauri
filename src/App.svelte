<script lang="ts">
  import Exporter from "./components/Exporter.svelte";
  import Settings from "./components/Settings.svelte";

  enum Action {
    EXPORT,
    TRANSLATION,
    QUERY,
    SETTINGS,
  }

  let action = Action.EXPORT;

  function switchAction(a: Action) {
    action = a;
  }
</script>

<main class="container">
  <div class="activitybar">
    <ul class="actions-container">
      <li
        class="action-item pointer"
        class:checked={action === Action.EXPORT}
        on:click={switchAction(Action.EXPORT)}
      >
        <a>BD导出</a>
      </li>
      <li
        class="action-item pointer"
        class:checked={action === Action.TRANSLATION}
        on:click={switchAction(Action.TRANSLATION)}
      >
        <a>物品翻译</a>
      </li>
      <li
        class="action-item pointer"
        class:checked={action === Action.QUERY}
        on:click={switchAction(Action.QUERY)}
      >
        <a>数据库</a>
      </li>
    </ul>
    <ul
      class="actions-container bottom"
      class:checked={action === Action.SETTINGS}
      on:click={switchAction(Action.SETTINGS)}
    >
      <li class="action-item pointer">设置</li>
    </ul>
  </div>
  <div class="content">
    {#if action === Action.EXPORT}
      <Exporter />
    {:else if action === Action.SETTINGS}
      <Settings />
    {/if}
  </div>
</main>

<style>
  main {
    display: flex;
    height: 100%;
  }

  .activitybar {
    width: 100px;
    background: #f5f5f5;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    flex: 0 0 auto;
  }

  .actions-container {
    margin: 20px 0;
  }

  .action-item {
    padding: 8px 0px;
    text-align: center;
    color: #475665;
  }

  .action-item a {
    font-size: 14px;
  }

  .checked {
    background: #fff;
    color: #000000;
  }

  .content {
    flex: 1 1 auto;
  }
</style>
