<script lang="ts">
  import { onMount } from "svelte";
  import { dockerRequest } from "../utils";

  import StackLine from "./StackLine.svelte";
  import { refreshContainers } from "../store";

  let stacks: any = {};
  let stats: any = {};

  async function getStats(id: string) {
    const prevStats = await dockerRequest(
      "GET",
      `/containers/${id}/stats?stream=false&one-shot=true`,
    );

    await new Promise((resolve) => setTimeout(() => resolve(null), 1000));

    const currentStats = await dockerRequest(
      "GET",
      `/containers/${id}/stats?stream=false&one-shot=true`,
    );

    const cpuDelta =
      prevStats.cpu_stats.cpu_usage.total_usage -
      currentStats.cpu_stats.cpu_usage.total_usage;

    const systemDelta =
      prevStats.cpu_stats.system_cpu_usage -
      currentStats.cpu_stats.system_cpu_usage;

    const nbCpu = currentStats.cpu_stats.online_cpus;

    const cpuUsage = (cpuDelta / systemDelta) * nbCpu * 100;

    stats[id] = {
      memory: currentStats?.memory_stats?.usage ?? 0,

      cpu: cpuUsage,
    };
  }

  async function updateContainerList() {
    const containers = await dockerRequest("GET", "/containers/json?all=true");

    let tmp: any = {};

    for (const container of containers) {
      const workingDir =
        container.Labels["com.docker.compose.project.working_dir"];

      if (!(workingDir in tmp)) {
        tmp[workingDir] = [];
      }

      if (container.State === "running") getStats(container.Id);
      else delete stats[container.Id];

      tmp[workingDir].push(container);
    }

    stacks = tmp;
  }

  $: if ($refreshContainers) {
    updateContainerList();
  }

  onMount(async () => {
    setInterval(() => {
      refreshContainers.set(Date.now());
    }, 5000);
  });
</script>

<div class="stacks">
  {#each Object.entries(stacks) as [index, containers] (index)}
    <StackLine name={index} {containers} {stats} />
  {/each}
</div>

<style lang="scss">
  .stacks {
    display: flex;
    flex-direction: column;
  }
</style>
