<script lang="ts">
  import { slide } from "svelte/transition";
  import { refreshContainers } from "../store";
  import { dockerRequest, sleep } from "../utils";
  import ContainerLine from "./ContainerLine.svelte";

  /* -------------------------------------------------------------------------- */

  export let name: string;
  export let containers: any;
  export let stats: any;

  /* -------------------------------------------------------------------------- */

  let isDisplayed = true;

  /* -------------------------------------------------------------------------- */

  async function stackAction(
    event: Event,
    action: string,
    filterState: string,
  ) {
    const target = event.target as HTMLButtonElement;

    target.disabled = true;
    for (const container of containers) {
      if (container.State === filterState) {
        continue;
      }
      try {
        await dockerRequest("POST", `/containers/${container.Id}/${action}`);
      } catch (err) {
        console.error(err);
      }
    }

    await sleep(200);
    target.disabled = false;
    refreshContainers.set(Date.now());
  }
</script>

<div>
  <button
    type="button"
    class="stack-line"
    on:click={() => {
      isDisplayed = !isDisplayed;
    }}
  >
    <div class="arrow-icon">
      {#if isDisplayed}
        <i class="ti ti-arrow-badge-up-filled"></i>
      {:else}
        <i class="ti ti-arrow-badge-down-filled"></i>
      {/if}
    </div>
    <div class="name">{name.slice(name.lastIndexOf("/") + 1)}</div>
    <div class="actions">
      <button
        type="button"
        on:click={(event) => stackAction(event, "start", "running")}
      >
        <i class="ti ti-player-play"></i>
      </button>
      <button
        type="button"
        on:click={(event) => stackAction(event, "stop", "exited")}
      >
        <i class="ti ti-player-stop"></i>
      </button>
      <button
        type="button"
        on:click={(event) => stackAction(event, "restart", "exited")}
      >
        <i class="ti ti-refresh"></i>
      </button>
      <!-- <button type="button">
        <i class="ti ti-trash"></i>
      </button> -->
    </div>
  </button>
  {#if isDisplayed}
    <div transition:slide={{ duration: 200 }}>
      {#each containers as container (container.Id)}
        <ContainerLine {container} stats={stats?.[container.Id] ?? {}} />
      {/each}
    </div>
  {/if}
</div>

<style lang="scss">
  .stack-line {
    all: unset;

    width: calc(100% - 20px);
    display: flex;

    align-items: center;
    height: 22px;
    padding: 8px 10px;

    justify-content: space-between;
    align-items: center;

    // background-color: #222;
    // border-bottom: 4px solid #222;

    margin-top: 4px;
    margin-bottom: 4px;

    // border-radius: 5px;

    &:hover {
      cursor: pointer;
    }

    .arrow-icon {
      display: flex;
      align-items: center;
      width: 22px;
    }

    > .name {
      font-weight: bold;
      flex-grow: 1;
    }

    > .actions {
      // width: 90px;
      display: flex;
      gap: 10px;

      > button {
        width: 32px;
        height: 32px;
        font-size: 16px;
        background-color: transparent;

        &:hover {
          background-color: #000000;
        }

        &:disabled {
          background-color: #242424;
          color: rgb(122, 122, 122);
          cursor: default;
        }
      }
    }
  }
</style>
