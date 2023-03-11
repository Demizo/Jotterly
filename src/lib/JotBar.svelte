<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import JotView from "$lib/JotView.svelte";

  let query = "";
  let jots = [{id: Number, text: String, img_path: String, time_create: String, time_modified: String }];
  let tags_list = [{id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}];
  async function search_jots() {
    if (query.trim().length > 2 || query.trim().length == 0 ) {
      jots = await invoke("search_jots", {query: query});
    }
  }
  let new_jot_id = Number;
  async function create_jot() {
    new_jot_id = await invoke("create_jot", {text: query, img_path: undefined});
    await search_jots();
  }
</script>



<div class="header" style="z-index: 100;">
  <h1>Jotterly</h1>
  <div class="row">
    {#if query.trim().length > 2}
      <!-- svelte-ignore a11y-autofocus -->
      <input autofocus inputmode="search" style="width: 450px;" on:keyup={search_jots} placeholder="Search or Jot..." bind:value={query} />
      <button on:click={create_jot} style="width: 50px;"><p class="thin-label">+</p></button>
    {:else}
      <!-- svelte-ignore a11y-autofocus -->
      <input autofocus inputmode="search" style="width: 500px;" on:keyup={search_jots} placeholder="Search or Jot..." bind:value={query} />
    {/if}
  </div>
</div>
{#await search_jots()}
  <!-- Show a loading indicator or message until the data is ready -->
  <p class="thin-label">Loading...</p>
{:then}
  <!-- The data is ready, so render it -->
  <div class="list">
  {#each jots as jot (jot.id)}
      <JotView search_jots={search_jots} jot={jot} bind:new_jot_id={new_jot_id}/>
  {:else}
  <p class="thin-label" style="margin-top: 1em;">No jots found...</p>
  {/each}
  </div>
{:catch error}
  <!-- Handle any errors that occur while fetching or processing the data -->
  <p>Error: {error.message}</p>
{/await}
<div class="footer">

</div>
<style>
  .header {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    height: 8em;
    padding: 10px;
    background: linear-gradient(to top, var(--backround-color-trans) 0%, var(--backround-color) 20%);
    /* box-sizing: border-box; */
  }
  .footer {
    position: fixed;
    top: calc(100vh - 2em);
    left: 0;
    right: 0;
    height: 2em;
    background: linear-gradient(to bottom, var(--backround-color-trans) 0%, var(--backround-color) 80%);
  }
  .list {
    margin-top: 8em;
    margin-bottom: 2em;
    /* overflow-y: auto; */
    /* max-height: calc(100vh - 10em); */
  }
</style>
