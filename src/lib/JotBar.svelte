<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import JotView from "$lib/JotView.svelte";
  import SettingsPopup from "$lib/SettingsPopup.svelte";
  import { register } from '@tauri-apps/api/globalShortcut';
  import { readText } from '@tauri-apps/api/clipboard';
  import { appWindow } from '@tauri-apps/api/window';

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

  register('CommandOrControl+Alt+Shift+J', () => {
    create_jot_from_clipboard();
  });
  async function create_jot_from_clipboard() {
    await appWindow.show();
    await appWindow.unminimize();
    await appWindow.setFocus();
    let text = await readText();
    new_jot_id = await invoke("create_jot", {text: text, img_path: undefined, tagIds: active_tags.map(t => t.id)});
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
  <svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px"
  width="287.77px" height="285.37px" viewBox="0 0 287.77 285.37" style="fill: var(--primary-color); margin-top: -5em; margin-bottom: -6em;"
  xml:space="preserve">
    <g id="Layer_7_copy">
      <g>
        <g>
          <g>
            <path class="st3" d="M41.54,131.33v-5.73c0-1.02,0.83-1.85,1.85-1.85h15.17c1.02,0,1.85,0.83,1.85,1.85v5.73
              c0,1.02-0.83,1.85-1.85,1.85H43.39C42.37,133.18,41.54,132.35,41.54,131.33z"/>
            <path class="st3" d="M60.41,118.48v-5.73c0-1.02,0.83-1.85,1.85-1.85H68c1.02,0,1.85,0.83,1.85,1.85v5.73
              c0,1.02-0.83,1.85-1.85,1.85h-5.73C61.24,120.34,60.41,119.51,60.41,118.48z"/>
            <path class="st3" d="M69.85,137.45v14.1c0,7.37-5.98,13.35-13.35,13.35h-1.61c-7.37,0-13.35-5.98-13.35-13.35v-4.66
              c0-0.48,0.39-0.86,0.86-0.86h7.72c0.48,0,0.86,0.39,0.86,0.86v3.86c0,2.61,2.11,4.72,4.72,4.72h0c2.61,0,4.72-2.11,4.72-4.72
              v-13.3c0-0.48,0.39-0.86,0.86-0.86h7.72C69.46,136.59,69.85,136.97,69.85,137.45z"/>
          </g>
        </g>
        <g>
          <path class="st3" d="M101.94,153.59c0,4.11-1.29,6.75-3.58,8.62c-2.05,1.64-4.58,2.64-7.39,2.64c-2.82,0-5.34-1-7.39-2.64
            c-2.29-1.88-3.58-4.52-3.58-8.62v-15.55c0-4.11,1.29-6.75,3.58-8.62c2.05-1.64,4.58-2.64,7.39-2.64c2.82,0,5.34,1,7.39,2.64
            c2.29,1.88,3.58,4.52,3.58,8.62V153.59z M93.79,138.04c0-1.17-0.29-1.94-0.76-2.46c-0.53-0.59-1.23-0.94-2.05-0.94
            s-1.53,0.35-2.05,0.94c-0.47,0.53-0.76,1.29-0.76,2.46v15.55c0,1.17,0.29,1.94,0.76,2.46c0.53,0.59,1.23,0.94,2.05,0.94
            s1.53-0.35,2.05-0.94c0.47-0.53,0.76-1.29,0.76-2.46V138.04z"/>
          <path class="st3" d="M122.07,164.44c-3.23,0-5.75-0.76-7.51-2.35c-1.7-1.53-2.64-3.87-2.64-7.22v-18.54
            c0-0.65-0.47-1.23-1.06-1.35l-0.65-0.12c-1-0.18-1.76-1.12-1.76-2.23v-3.17c0-1.11,0.76-2.05,1.76-2.23l0.65-0.12
            c0.59-0.12,1.06-0.7,1.06-1.35v-3.4c0-1.29,1-2.29,2.29-2.29h3.58c1.29,0,2.29,1,2.29,2.29v3.4c0,0.76,0.53,1.41,1.41,1.41h1.58
            c1.29,0,2.29,1,2.29,2.29v3.17c0,1.29-1,2.29-2.29,2.29h-1.53c-0.88,0-1.47,0.59-1.47,1.35v17.25c0,1.82,0.82,2.82,2.23,2.93
            l0.76,0.06c1.29,0.12,2.29,1,2.29,2.29v3.34c0,1.29-1,2.29-2.29,2.29H122.07z"/>
          <path class="st3" d="M145.37,164.44c-3.23,0-5.75-0.76-7.51-2.35c-1.7-1.53-2.64-3.87-2.64-7.22v-18.54
            c0-0.65-0.47-1.23-1.06-1.35l-0.65-0.12c-1-0.18-1.76-1.12-1.76-2.23v-3.17c0-1.11,0.76-2.05,1.76-2.23l0.65-0.12
            c0.59-0.12,1.06-0.7,1.06-1.35v-3.4c0-1.29,1-2.29,2.29-2.29h3.58c1.29,0,2.29,1,2.29,2.29v3.4c0,0.76,0.53,1.41,1.41,1.41h1.58
            c1.29,0,2.29,1,2.29,2.29v3.17c0,1.29-1,2.29-2.29,2.29h-1.53c-0.88,0-1.47,0.59-1.47,1.35v17.25c0,1.82,0.82,2.82,2.23,2.93
            l0.76,0.06c1.29,0.12,2.29,1,2.29,2.29v3.34c0,1.29-1,2.29-2.29,2.29H145.37z"/>
          <path class="st3" d="M177.11,153.88c0,3.81-1.29,6.45-3.58,8.33c-2.05,1.64-4.58,2.64-7.39,2.64c-2.82,0-5.34-1-7.39-2.64
            c-2.29-1.88-3.58-4.52-3.58-8.62v-15.55c0-4.11,1.29-6.75,3.58-8.62c2.05-1.64,4.58-2.64,7.39-2.64c2.82,0,5.34,1,7.39,2.64
            c2.29,1.88,3.58,4.52,3.58,8.33v7.39c0,1.47-1.17,2.93-2.93,2.93h-9.45c-0.82,0-1.41,0.59-1.41,1.35v4.34
            c0,1.17,0.29,1.82,0.76,2.35c0.53,0.59,1.23,0.94,2.05,0.94c0.7,0,1.41-0.29,1.94-0.76c0.53-0.53,0.88-1.23,0.88-2.52v-0.59
            c0-1.29,1-2.29,2.29-2.29h3.58c1.29,0,2.29,1,2.29,2.29V153.88z M163.32,139.74c0,0.76,0.59,1.35,1.41,1.35h2.7
            c0.88,0,1.53-0.59,1.53-1.35v-1.94c0-1.17-0.35-1.88-0.88-2.41c-0.53-0.47-1.12-0.76-1.94-0.76s-1.53,0.35-2.05,0.94
            c-0.47,0.53-0.76,1.17-0.76,2.35V139.74z"/>
          <path class="st3" d="M187.85,164.44c-1.29,0-2.29-1-2.29-2.29v-32.5c0-1,0.35-1.7,0.88-2.17c0.47-0.41,1.12-0.65,1.82-0.65
            c0.82,0,1.53,0.35,2.11,0.82l0.23,0.18c0.35,0.29,0.82,0.53,1.35,0.53c0.65,0,1.35-0.53,2.41-0.94c1.06-0.41,1.94-0.59,2.82-0.59
            c1.76,0,2.46,1.11,2.46,2.41v4.22c0,1.29-1,2.29-2.29,2.29h-1.53c-0.65,0-1.17,0.18-1.53,0.59c-0.47,0.41-0.59,0.94-0.59,2.35
            v23.47c0,1.29-1,2.29-2.29,2.29H187.85z"/>
          <path class="st3" d="M215.78,162.15c0,1.29-1,2.29-2.29,2.29h-3.58c-1.29,0-2.29-1-2.29-2.29v-43.83c0-1.29,1-2.29,2.29-2.29
            h3.58c1.29,0,2.29,1,2.29,2.29V162.15z"/>
          <path class="st3" d="M246.23,163.85c0,3.75-1.35,6.45-3.7,8.27c-1.94,1.47-4.34,2.35-7.16,2.35s-5.46-0.94-7.51-2.52
            c-2.29-1.82-3.52-4.11-3.52-7.33c0-0.12,0-0.18,0-0.29c0.12-1.17,1.12-2,2.29-2h3.58c1.23,0,2.11,0.82,2.29,2.11l0.06,0.29
            c0.12,0.53,0.35,1,0.7,1.29c0.41,0.41,1.06,0.59,1.88,0.59c1,0,1.7-0.35,2.23-0.94c0.47-0.53,0.76-1.23,0.76-2.41v-1.41
            c0-0.47-0.18-0.88-0.59-1.12c-0.23-0.12-0.53-0.23-0.88-0.23h-1.12c-1.47,0-1.7-0.06-1.99-0.06c-1.82-0.12-3.81-0.7-5.75-2.23
            c-2.93-2.35-3.52-5.93-3.52-8.39v-20.36c0-1.29,1-2.29,2.29-2.29h3.58c1.29,0,2.29,1,2.29,2.41v20.42c0,1.29,0.35,1.99,0.88,2.52
            c0.53,0.47,1.23,0.76,1.94,0.76h1.35c0.88,0,1.47-0.59,1.47-1.35v-22.47c0-1.29,1-2.29,2.29-2.29h3.58c1.29,0,2.29,1,2.29,2.29
            V163.85z"/>
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
