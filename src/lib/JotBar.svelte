<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import JotView from "$lib/JotView.svelte";
  import EditJotPopup from './EditJotPopup.svelte';

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



<div class="header">
  <h1>Jotterly</h1>
  <div class="row">
    <input autofocus inputmode="search" on:keyup={search_jots} placeholder="Search or Jot..." bind:value={query} />
  </div>
</div>
{#await search_jots()}
  <!-- Show a loading indicator or message until the data is ready -->
  <p>Loading...</p>
{:then}
  <!-- The data is ready, so render it -->
  {#if query.trim().length > 2}
    <button on:click={create_jot}>New Jot + "{query}"</button>
  {/if}
  {#each jots as jot (jot.id)}
      <JotView search_jots={search_jots} jot={jot} bind:new_jot_id={new_jot_id}/>
  {:else}
    No Jots
  {/each}
{:catch error}
  <!-- Handle any errors that occur while fetching or processing the data -->
  <p>Error: {error.message}</p>
{/await}

<style>
  .header {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    height: 50px;
    padding: 10px;
    box-sizing: border-box;
  }
</style>
