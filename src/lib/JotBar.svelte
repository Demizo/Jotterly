<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import JotView from "$lib/JotView.svelte";

  let query: String = "";
  let jots: {id: Number, text: String, img_path: String, time_create: String, time_modified: String }[] = [];
  let active_tags: {id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}[] = [];
  let tags_list: {id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}[] = [];

  async function search_jots() {
    tags_list = await invoke("get_top_tags");
    if(active_tags.length > 0) {
      tags_list = tags_list.filter(t => !active_tags.map(t => t.title).includes(t.title));
    }
    console.log(active_tags.map(t => t.id));
    jots = await invoke("search_jots", {query: query, activeTags: active_tags.map(t => t.id)});
    update_height();
  }

  let new_jot_id = Number;
  async function create_jot() {
    new_jot_id = await invoke("create_jot", {text: query, img_path: undefined, tagIds: active_tags.map(t => t.id)});
    await search_jots();
  }

  function add_tag(tag: {id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}){
      active_tags.push(tag);
      active_tags = [...active_tags];
  }
  function remove_tag(title: String){
    active_tags = active_tags.filter(t => t.title != title);
  }
  let header_height = document.getElementById('header')?.clientHeight;
  function update_height(){
    header_height = document.getElementById('header')?.clientHeight;
  }
</script>



<div id="header" class="header" style="z-index: 100;">
  <h1>Jotterly</h1>
  <div class="row">
     <div>
      {#if active_tags.length > 0}
      <div class="active-tags-list">
        {#each active_tags as tag, i}
          <button tabindex="-1" class="tag remove-tag" on:click={() => {
              remove_tag(tag.title);
              search_jots();
          }}>
            <p>{tag.title}</p>
            <svg class="remove-icon" xmlns="http://www.w3.org/2000/svg" height="16px" viewBox="0 0 24 24" width="16px" fill="#000000">
              <path d="M0 0h24v24H0V0z" fill="none"/><path d="M18.3 5.71c-.39-.39-1.02-.39-1.41 0L12 10.59 7.11 5.7c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41L10.59 12 5.7 16.89c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0L12 13.41l4.89 4.89c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L13.41 12l4.89-4.89c.38-.38.38-1.02 0-1.4z"/>
            </svg>
          </button>
        {/each}
      </div>
      {/if}
      <div class="search-area">
        <!-- svelte-ignore a11y-autofocus -->
        <textarea class="search-box" autofocus inputmode="text" style="width: 500px;" on:keyup={search_jots} placeholder="Search or Jot..." bind:value={query} />
        {#if query.trim().length > 2}
          <button on:click={create_jot} class="new-jot-button" style="width: 50px;">
            <!-- <img src="/add_circle_black_24dp.svg" alt="+"> -->
            <svg class="new-jot-icon" xmlns="http://www.w3.org/2000/svg" height="32px" viewBox="0 0 24 24" width="32px" fill="var(--foreground-font-color-secondary)">
              <path d="M0 0h24v24H0V0z" fill="none"/><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm4 11h-3v3c0 .55-.45 1-1 1s-1-.45-1-1v-3H8c-.55 0-1-.45-1-1s.45-1 1-1h3V8c0-.55.45-1 1-1s1 .45 1 1v3h3c.55 0 1 .45 1 1s-.45 1-1 1z"/>
            </svg>
          </button>
        {/if}
      </div>
      <div class="tags-list">
        {#each tags_list as tag, i}
          <button tabindex="-1" class="tag apply-tag" on:click={() => {
              add_tag(tag);
              search_jots();
          }}>
            <p>{tag.title}</p>
            <svg class="apply-icon" xmlns="http://www.w3.org/2000/svg" height="16px" viewBox="0 0 24 24" width="16px" fill="#000000">
              <path d="M0 0h24v24H0V0z" fill="none"/><path d="M18 13h-5v5c0 .55-.45 1-1 1s-1-.45-1-1v-5H6c-.55 0-1-.45-1-1s.45-1 1-1h5V6c0-.55.45-1 1-1s1 .45 1 1v5h5c.55 0 1 .45 1 1s-.45 1-1 1z"/>
            </svg>
          </button>
        {/each}
      </div>
     </div>
  </div>
</div>
{#await search_jots()}
  <!-- Show a loading indicator or message until the data is ready -->
  <p class="thin-label">Loading...</p>
{:then}
  <!-- The data is ready, so render it -->
  <div id="list" class="list" style="margin-top: {header_height}px;">
    {#if query.trim().length < 1}
  <p class="thin-label">Recent Jots</p>
  {/if}
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
    padding: 10px;
    padding-bottom: 2em;
    background: linear-gradient(to top, var(--backround-color-trans) 0%, var(--backround-color) 10%);
    /* box-sizing: border-box; */
  }
  .search-box {
    min-height: 1em;
    height: 2.5em;
    margin-bottom: -0.5em;
    margin-top: 0.75em;
  }
  .tags-list {
    text-align: left;
    width: 500px;
    height:40px;
    overflow: clip;
  }
  ::-webkit-scrollbar {
    display: none;
  }
  .active-tags-list {
    text-align: left;
    width: 500px;
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
    margin-bottom: 2em;
    /* overflow-y: auto; */
    /* max-height: calc(100vh - 10em); */
  }
  .search-area {
    position: relative;
    display: inline-block;
  }
  .new-jot-button {
    position: absolute;
    bottom: -0.25em;
    right: -0.75em;
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
