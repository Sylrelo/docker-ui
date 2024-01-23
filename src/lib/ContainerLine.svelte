<script lang="ts">
  import { dockerRequest } from "../utils";
  import { refreshContainers } from "../store";

  export let container: any;
  export let stats: any;

  let isStarting = false;
  let isStopping = false;

  let exposedPorts: any[] = [];

  async function stopContainer(id: string) {
    console.log("STOP");
    isStopping = true;
    const result = await dockerRequest("POST", `/containers/${id}/stop`);
    isStopping = false;
    // console.log("DONE");
    refreshContainers.set(Date.now());
  }

  async function startContainer(id: string) {
    console.log("START");
    isStarting = true;
    const result = await dockerRequest("POST", `/containers/${id}/start`);
    isStarting = false;
    // console.log("DONE");
    refreshContainers.set(Date.now());
  }

  async function restartContainer(id: string) {
    console.log("START");
    const result = await dockerRequest("POST", `/containers/${id}/restart`);
    // console.log("DONE");
    refreshContainers.set(Date.now());
  }

  $: if (container.Ports) {
    exposedPorts = container.Ports.filter((port: any) => port.PublicPort).sort(
      (a: any, b: any) => a.PublicPort - b.PublicPort,
    );
  }
</script>

<div class="container-line">
  <div class="principal">
    <div
      class="state {container.State}"
      class:is-starting={isStarting}
      class:is-stopping={isStopping}
    ></div>
    <div class="name">
      <div>{container.Names[0].slice(1)}</div>
      <div style="font-size: 10px; margin-top: -6px">{container.Image}</div>
    </div>

    <div class="cpu">{(stats?.cpu ?? 0).toFixed(1)} %</div>
    <div class="ram">
      {((stats?.memory ?? 0) / 1000 / 1000).toFixed(1)} Mb
    </div>
    <!-- <div class="cpu">00.0 %</div> -->
    <!-- <div class="ram">000.0 Gb</div> -->
    <!-- <div class="net">484 Ko/s</div>
  <div class="io">484 Ko/s</div> -->

    <div class="actions">
      <button
        type="button"
        disabled={container.State === "running"}
        on:click={async (event) => {
          //@ts-ignore
          event.target.disabled = true;
          await startContainer(container.Id);
          //@ts-ignore
          event.target.disabled = false;
        }}
      >
        <i class="ti ti-player-play"></i>
      </button>
      <button
        type="button"
        disabled={container.State === "exited"}
        on:click={async (event) => {
          //@ts-ignore
          event.target.disabled = true;
          await stopContainer(container.Id);
          //@ts-ignore
          event.target.disabled = false;
        }}
      >
        <i class="ti ti-player-stop"></i>
      </button>
      <button
        type="button"
        disabled={container.State === "exited"}
        on:click={async (event) => {
          //@ts-ignore
          event.target.disabled = true;
          await restartContainer(container.Id);
          //@ts-ignore
          event.target.disabled = false;
        }}
      >
        <i class="ti ti-refresh"></i>
      </button>
      <!-- <button type="button" disabled={true || container.State === "exited"}>
        <i class="ti ti-terminal"></i>
      </button>
      <button type="button" disabled={true}>
        <i class="ti ti-folder"></i>
      </button> -->
      <!-- <button type="button" disabled={true || container.State === "running"}>
        <i class="ti ti-trash"></i>
      </button> -->
    </div>
  </div>

  {#if exposedPorts.length && container.State === "running"}
    <div class="secondary">
      <div class="ports">
        {#each exposedPorts as port}
          <div>
            <a href="http://localhost:{port.PublicPort}" target="_blank">
              {port.PublicPort} -> {port.PrivatePort}
            </a>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style lang="scss">
  @keyframes blinker {
    50% {
      opacity: 0;
    }
  }

  .container-line {
    display: flex;
    flex-direction: column;
    gap: 2px;

    margin-left: 20px;
    margin-bottom: 4px;
    padding: 8px;
    background-color: #292929;
    border-radius: 5px;
  }

  .principal,
  .secondary {
    display: flex;
    gap: 4px;

    align-items: center;
  }

  .secondary {
    justify-content: start;
    font-size: 12px;

    > .ports {
      display: flex;
      gap: 4px;
      a {
        all: unset;
        text-decoration: underline;
        &:hover {
          color: #99b8ee;
        }
      }
      > div {
        background-color: #202020;
        padding: 2px 6px;
        border-radius: 5px;
      }
    }
  }

  .principal {
    justify-content: space-between;

    > .cpu,
    .ram,
    .io,
    .net {
      text-align: center;
      width: 60px;
      font-size: 12px;
      // border: 1px solid red;
    }

    > .cpu {
      width: 50px;
    }

    > .actions {
      display: flex;
      gap: 4px;

      > button {
        font-size: 14px;

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

    > .name {
      flex-grow: 1;
      // display: flex;
      // flex-direction: column;
    }

    > .state {
      width: 10px;
      height: 10px;
      border-radius: 5px;

      margin-right: 4px;
      overflow: hidden;

      background-color: rgb(160, 36, 36);

      transition: background-color 0.2s ease-in-out;

      &.running {
        background-color: rgb(36, 160, 50);
      }

      &.is-stopping {
        background-color: red;
        animation: blinker 1s linear infinite;
      }

      &.is-starting {
        background-color: rgb(50, 135, 203);
        animation: blinker 1s linear infinite;
      }
    }
  }
</style>
