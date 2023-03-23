<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import JotView from "$lib/JotView.svelte";
  import SettingsPopup from "$lib/SettingsPopup.svelte";
  
  let showSettings = false;
  function openSettings() {
    showSettings=true;
    document.body.style.overflow = 'hidden';
  }
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

<SettingsPopup bind:visible={showSettings}></SettingsPopup>
<div id="header" class="header" style="z-index: 100;">
  <svg
    version="1.1"
    x="0px"
    y="0px"
    width="181.41"
    height="63.57"
    viewBox="0 0 181.41001 63.570001"
    xml:space="preserve"
    id="svg38"
    xmlns="http://www.w3.org/2000/svg"
    style="fill: var(--primary-color); margin-top: 1em;"><defs id="defs42" />
    <g id="Layer_7_copy" transform="translate(-50.160002,-110.9)">
      <g id="g35">
        <g id="g19">
          <g id="g17">
            <path class="st3"
      d="m 50.16,131.33 v -5.73 c 0,-1.02 0.83,-1.85 1.85,-1.85 h 15.17 c 1.02,0 1.85,0.83 1.85,1.85 v 5.73 c 0,1.02 -0.83,1.85 -1.85,1.85 H 52.02 c -1.03,0 -1.86,-0.83 -1.86,-1.85 z"
      id="path11" />
            <path
      class="st3"
      d="m 69.04,118.48 v -5.73 c 0,-1.02 0.83,-1.85 1.85,-1.85 h 5.73 c 1.02,0 1.85,0.83 1.85,1.85 v 5.73 c 0,1.02 -0.83,1.85 -1.85,1.85 h -5.73 c -1.02,0.01 -1.85,-0.82 -1.85,-1.85 z"
      id="path13" />
            <path
      class="st3"
      d="m 78.47,137.45 v 14.1 c 0,7.37 -5.98,13.35 -13.35,13.35 h -1.61 c -7.37,0 -13.35,-5.98 -13.35,-13.35 v -4.66 c 0,-0.48 0.39,-0.86 0.86,-0.86 h 7.72 c 0.48,0 0.86,0.39 0.86,0.86 v 3.86 c 0,2.61 2.11,4.72 4.72,4.72 v 0 c 2.61,0 4.72,-2.11 4.72,-4.72 v -13.3 c 0,-0.48 0.39,-0.86 0.86,-0.86 h 7.72 c 0.47,0 0.85,0.38 0.85,0.86 z"
      id="path15" />
          </g>
        </g>
        <g
      id="g33">
          <path
      class="st3"
      d="m 110.57,153.59 c 0,4.11 -1.29,6.75 -3.58,8.62 -2.05,1.64 -4.58,2.64 -7.39,2.64 -2.81,0 -5.34,-1 -7.39,-2.64 -2.29,-1.88 -3.58,-4.52 -3.58,-8.62 v -15.55 c 0,-4.11 1.29,-6.75 3.58,-8.62 2.05,-1.64 4.58,-2.64 7.39,-2.64 2.81,0 5.34,1 7.39,2.64 2.29,1.88 3.58,4.52 3.58,8.62 z m -8.16,-15.55 c 0,-1.17 -0.29,-1.94 -0.76,-2.46 -0.53,-0.59 -1.23,-0.94 -2.05,-0.94 -0.82,0 -1.53,0.35 -2.05,0.94 -0.47,0.53 -0.76,1.29 -0.76,2.46 v 15.55 c 0,1.17 0.29,1.94 0.76,2.46 0.53,0.59 1.23,0.94 2.05,0.94 0.82,0 1.53,-0.35 2.05,-0.94 0.47,-0.53 0.76,-1.29 0.76,-2.46 z"
      id="path21" />
          <path
      class="st3"
      d="m 130.7,164.44 c -3.23,0 -5.75,-0.76 -7.51,-2.35 -1.7,-1.53 -2.64,-3.87 -2.64,-7.22 v -18.54 c 0,-0.65 -0.47,-1.23 -1.06,-1.35 l -0.65,-0.12 c -1,-0.18 -1.76,-1.12 -1.76,-2.23 v -3.17 c 0,-1.11 0.76,-2.05 1.76,-2.23 l 0.65,-0.12 c 0.59,-0.12 1.06,-0.7 1.06,-1.35 v -3.4 c 0,-1.29 1,-2.29 2.29,-2.29 h 3.58 c 1.29,0 2.29,1 2.29,2.29 v 3.4 c 0,0.76 0.53,1.41 1.41,1.41 h 1.58 c 1.29,0 2.29,1 2.29,2.29 v 3.17 c 0,1.29 -1,2.29 -2.29,2.29 h -1.53 c -0.88,0 -1.47,0.59 -1.47,1.35 v 17.25 c 0,1.82 0.82,2.82 2.23,2.93 l 0.76,0.06 c 1.29,0.12 2.29,1 2.29,2.29 v 3.34 c 0,1.29 -1,2.29 -2.29,2.29 h -0.99 z"
      id="path23" />
          <path
      class="st3"
      d="m 162.44,153.88 c 0,3.81 -1.29,6.45 -3.58,8.33 -2.05,1.64 -4.58,2.64 -7.39,2.64 -2.81,0 -5.34,-1 -7.39,-2.64 -2.29,-1.88 -3.58,-4.52 -3.58,-8.62 v -15.55 c 0,-4.11 1.29,-6.75 3.58,-8.62 2.05,-1.64 4.58,-2.64 7.39,-2.64 2.81,0 5.34,1 7.39,2.64 2.29,1.88 3.58,4.52 3.58,8.33 v 7.39 c 0,1.47 -1.17,2.93 -2.93,2.93 h -9.45 c -0.82,0 -1.41,0.59 -1.41,1.35 v 4.34 c 0,1.17 0.29,1.82 0.76,2.35 0.53,0.59 1.23,0.94 2.05,0.94 0.7,0 1.41,-0.29 1.94,-0.76 0.53,-0.53 0.88,-1.23 0.88,-2.52 v -0.59 c 0,-1.29 1,-2.29 2.29,-2.29 h 3.58 c 1.29,0 2.29,1 2.29,2.29 z m -13.79,-14.14 c 0,0.76 0.59,1.35 1.41,1.35 h 2.7 c 0.88,0 1.53,-0.59 1.53,-1.35 v -1.94 c 0,-1.17 -0.35,-1.88 -0.88,-2.41 -0.53,-0.47 -1.12,-0.76 -1.94,-0.76 -0.82,0 -1.53,0.35 -2.05,0.94 -0.47,0.53 -0.76,1.17 -0.76,2.35 v 1.82 z"
      id="path25" />
          <path
      class="st3"
      d="m 173.18,164.44 c -1.29,0 -2.29,-1 -2.29,-2.29 v -32.5 c 0,-1 0.35,-1.7 0.88,-2.17 0.47,-0.41 1.12,-0.65 1.82,-0.65 0.82,0 1.53,0.35 2.11,0.82 l 0.23,0.18 c 0.35,0.29 0.82,0.53 1.35,0.53 0.65,0 1.35,-0.53 2.41,-0.94 1.06,-0.41 1.94,-0.59 2.82,-0.59 1.76,0 2.46,1.11 2.46,2.41 v 4.22 c 0,1.29 -1,2.29 -2.29,2.29 h -1.53 c -0.65,0 -1.17,0.18 -1.53,0.59 -0.47,0.41 -0.59,0.94 -0.59,2.35 v 23.47 c 0,1.29 -1,2.29 -2.29,2.29 h -3.56 z"
      id="path27" />
          <path
      class="st3"
      d="m 201.11,162.15 c 0,1.29 -1,2.29 -2.29,2.29 h -3.58 c -1.29,0 -2.29,-1 -2.29,-2.29 v -43.83 c 0,-1.29 1,-2.29 2.29,-2.29 h 3.58 c 1.29,0 2.29,1 2.29,2.29 z"
      id="path29" />
          <path
      class="st3"
      d="m 231.56,163.85 c 0,3.75 -1.35,6.45 -3.7,8.27 -1.94,1.47 -4.34,2.35 -7.16,2.35 -2.82,0 -5.46,-0.94 -7.51,-2.52 -2.29,-1.82 -3.52,-4.11 -3.52,-7.33 0,-0.12 0,-0.18 0,-0.29 0.12,-1.17 1.12,-2 2.29,-2 h 3.58 c 1.23,0 2.11,0.82 2.29,2.11 l 0.06,0.29 c 0.12,0.53 0.35,1 0.7,1.29 0.41,0.41 1.06,0.59 1.88,0.59 1,0 1.7,-0.35 2.23,-0.94 0.47,-0.53 0.76,-1.23 0.76,-2.41 v -1.41 c 0,-0.47 -0.18,-0.88 -0.59,-1.12 -0.23,-0.12 -0.53,-0.23 -0.88,-0.23 h -1.12 c -1.47,0 -1.7,-0.06 -1.99,-0.06 -1.82,-0.12 -3.81,-0.7 -5.75,-2.23 -2.93,-2.35 -3.52,-5.93 -3.52,-8.39 v -20.36 c 0,-1.29 1,-2.29 2.29,-2.29 h 3.58 c 1.29,0 2.29,1 2.29,2.41 V 150 c 0,1.29 0.35,1.99 0.88,2.52 0.53,0.47 1.23,0.76 1.94,0.76 h 1.35 c 0.88,0 1.47,-0.59 1.47,-1.35 v -22.47 c 0,-1.29 1,-2.29 2.29,-2.29 h 3.58 c 1.29,0 2.29,1 2.29,2.29 v 34.39 z"
      id="path31" />
        </g>
      </g>
    </g>
  </svg>
  <button tabindex="-1" class="settings-button" on:click={openSettings}>
    <svg class="settings-icon" xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="32px" viewBox="0 0 24 24" width="32px" fill="#000000">
      <rect fill="none" height="24" width="24"/><path d="M19.5,12c0-0.23-0.01-0.45-0.03-0.68l1.86-1.41c0.4-0.3,0.51-0.86,0.26-1.3l-1.87-3.23c-0.25-0.44-0.79-0.62-1.25-0.42 l-2.15,0.91c-0.37-0.26-0.76-0.49-1.17-0.68l-0.29-2.31C14.8,2.38,14.37,2,13.87,2h-3.73C9.63,2,9.2,2.38,9.14,2.88L8.85,5.19 c-0.41,0.19-0.8,0.42-1.17,0.68L5.53,4.96c-0.46-0.2-1-0.02-1.25,0.42L2.41,8.62c-0.25,0.44-0.14,0.99,0.26,1.3l1.86,1.41 C4.51,11.55,4.5,11.77,4.5,12s0.01,0.45,0.03,0.68l-1.86,1.41c-0.4,0.3-0.51,0.86-0.26,1.3l1.87,3.23c0.25,0.44,0.79,0.62,1.25,0.42 l2.15-0.91c0.37,0.26,0.76,0.49,1.17,0.68l0.29,2.31C9.2,21.62,9.63,22,10.13,22h3.73c0.5,0,0.93-0.38,0.99-0.88l0.29-2.31 c0.41-0.19,0.8-0.42,1.17-0.68l2.15,0.91c0.46,0.2,1,0.02,1.25-0.42l1.87-3.23c0.25-0.44,0.14-0.99-0.26-1.3l-1.86-1.41 C19.49,12.45,19.5,12.23,19.5,12z M12.04,15.5c-1.93,0-3.5-1.57-3.5-3.5s1.57-3.5,3.5-3.5s3.5,1.57,3.5,3.5S13.97,15.5,12.04,15.5z"/>
  </button>
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
            <svg class="new-jot-icon" xmlns="http://www.w3.org/2000/svg" height="32px" viewBox="0 0 24 24" width="32px" fill="var(--font-color-secondary)">
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
  {#if query.trim().length < 1 && jots.length > 0 && active_tags.length == 0}
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
    background: linear-gradient(to top, var(--background-color-trans) 0%, var(--background-color) 10%);
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
    background: linear-gradient(to bottom, var(--background-color-trans) 0%, var(--background-color) 80%);
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
  .settings-button {
    position: absolute;
    top: 1em;
    right: 1em;
    border: none;
    background: none;
    box-shadow: none;
    text-align: justify;
    padding: 0em;
    margin: 0em;
  }
  .settings-icon {
    fill: var(--font-color-secondary);
  }
  button:hover .settings-icon{
    fill: var(--primary-color);
    transition: fill 0.25s;
  }
  button:focus .settings-icon{
    fill: var(--primary-color);
    transition: fill 0.25s;
  }
</style>
