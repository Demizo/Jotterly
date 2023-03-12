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
      <button on:click={create_jot} class="new-jot-button" style="width: 50px;">
        <!-- <img src="/add_circle_black_24dp.svg" alt="+"> -->
        <svg class="new-jot-icon" xmlns="http://www.w3.org/2000/svg" height="42px" viewBox="0 0 24 24" width="42px" fill="var(--foreground-font-color-secondary)">
          <g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g/><g><path d="M18,12c-0.55,0-1,0.45-1,1v5.22c0,0.55-0.45,1-1,1H6c-0.55,0-1-0.45-1-1V8c0-0.55,0.45-1,1-1h5c0.55,0,1-0.45,1-1 c0-0.55-0.45-1-1-1H5C3.9,5,3,5.9,3,7v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-6C19,12.45,18.55,12,18,12z"/><path d="M21.02,5H19V2.98C19,2.44,18.56,2,18.02,2h-0.03C17.44,2,17,2.44,17,2.98V5h-2.01C14.45,5,14.01,5.44,14,5.98 c0,0.01,0,0.02,0,0.03C14,6.56,14.44,7,14.99,7H17v2.01c0,0.54,0.44,0.99,0.99,0.98c0.01,0,0.02,0,0.03,0 c0.54,0,0.98-0.44,0.98-0.98V7h2.02C21.56,7,22,6.56,22,6.02V5.98C22,5.44,21.56,5,21.02,5z"/><path d="M14,9H8c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1C15,9.45,14.55,9,14,9z"/><path d="M14,12H8c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1C15,12.45,14.55,12,14,12z"/><path d="M14,15H8c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1C15,15.45,14.55,15,14,15z"/></g></g>
        </svg>
      </button>
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
  .new-jot-button {
    border: none;
    background: none;
    box-shadow: none;
    text-align: justify;
    padding: 0em;
    margin: 0em;
    width: 50px;
    height: 46px;
  }
  button:hover .new-jot-icon{
    fill: var(--primary-color);
    transition: fill 0.25s;
  }
  button:focus .new-jot-icon{
    fill: var(--primary-color);
    transition: fill 0.25s;
  }
</style>
