<script lang="ts">
</script>

<div class="menu">
  <button type="button" class="active">
    <i class="ti ti-layout-grid"></i>
  </button>

  <button type="button" disabled={true}>
    <i class="ti ti-device-laptop"></i>
  </button>

  <button type="button" disabled={true}>
    <i class="ti ti-server"></i>
  </button>

  <button type="button" disabled={true}>
    <i class="ti ti-network"></i>
  </button>
</div>

<style lang="scss">
  .menu {
    position: absolute;

    padding: 0px 10px;
    width: calc(64px - 20px);
    height: 100vh;

    background-color: #222;

    display: flex;
    flex-direction: column;
    // gap: 10px;

    > button {
      // all: unset;
      margin-top: 10px;
      display: flex;
      justify-content: center;
      align-items: center;
      padding: 4px 4px;
      font-size: 32px;

      // &.active {
      //   background-color: transparent;
      // }

      &:hover {
        background-color: #333;
      }

      &:disabled {
        background-color: #333;
        background-color: transparent;
        color: #535353;
        cursor: default;
      }
    }
  }
</style>
