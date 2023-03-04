<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { focusTrap } from 'svelte-focus-trap';

    export let focusText = true;
    export let visible = false;
    export let jotId: Number;
    export let jot = {id: Number, text: String, img_path: String, time_create: String, time_modified: String };
    export let search_jots;
    export let tags = [{id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}];
    let query = "";
    let pending = false;
    let text = jot.text;
    let temptext = text;
    
    function closePopup() {
      query = "";
      visible = false;
    }
    let tags_list: {id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}[];
    async function search_tags() {
      if (query.trim().length > 2 || query.trim().length == 0 && pending == false) {
        let tag_ids = tags.map(tag => tag.id);
        console.log(tag_ids);
        tags_list = await invoke("search_tags", {query: query, tagIds: tag_ids}).catch(() => console.log("failed to get tags")) as {id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}[];
        console.log(tags_list);
      }
    }
    async function add_tag_to_jot(tag_id: Number) {
      await invoke("add_tag_to_jot", {tagId: tag_id, jotId: jotId}).catch(() => console.log("failed to add tag"));
    }
    async function remove_tag_from_jot(tag_id: Number) {
      pending = true;
      await invoke("remove_tag_from_jot", {tagId: tag_id, jotId: jotId}).catch(() => console.log("failed to remove tag"));
      pending = false;
      search_tags();
    }
    async function add_new_tag_to_jot() {
      let tag = await invoke("add_new_tag_to_jot", {title: query, jotId: jotId}).catch(() => console.log("failed to add new tag")) as {id: NumberConstructor, title: StringConstructor, color: StringConstructor, priority: NumberConstructor, time_create: StringConstructor, time_modified: StringConstructor};
      tags.push(tag);
      tags = [...tags];
      search_tags();
    }
    function saveChanges() {
      text = temptext;
      jot.text = text;
      save_jot_text();
    }
    async function save_jot_text() {
      await invoke("update_jot_text", {id: jot.id, text: text, img_path: jot.img_path});
    }
    async function delete_jot() {
      await invoke("delete_jot", {id: jot.id});
      search_jots();
    }
    function calcTextAreaHeight(event) {
      const textarea = event.target;
      textarea.style.height = "auto";
      textarea.style.height = `${textarea.scrollHeight}px`;
    }
    
  </script>
  
  {#if visible}
    <div class="popup">
      <div class="popup-content" use:focusTrap>
        <div class="action-buttons">
          <div></div>
          <div>
            <button class="action-button" on:click={delete_jot}>d</button>
            <button disabled={temptext.toString().trim().length <= 0 || temptext === text} class="action-button" on:click={saveChanges}>s</button>
            <button class="action-button" on:click={closePopup}>x</button>
          </div>
        </div>
        {#if focusText}
          <textarea autofocus on:select={calcTextAreaHeight} on:input={calcTextAreaHeight} bind:value={temptext}></textarea>
        {:else}
        <textarea on:select={calcTextAreaHeight} on:input={calcTextAreaHeight} bind:value={temptext}></textarea>
        {/if}
        <div class="left-row">
          <p class="thin-label">Remove Tags...</p>
        </div>
        {#each tags as tag, i}
          <button on:click={() => {
            remove_tag_from_jot(tag.id)
            // force reload of tags list
            tags = tags.filter(t => t.id != tag.id);
            search_tags();
          }} class="tag">{tag.title}</button>
        {:else}
          <p>No tags...</p>
        {/each}
        {#if focusText}
          <input style="width: 27em; margin-bottom: 1em;" inputmode="search" on:keyup={search_tags} placeholder="Search..." bind:value={query}/>
        {:else}
          <input autofocus style="width: 27em; margin-bottom: 1em;" inputmode="search" on:keyup={search_tags} placeholder="Search..." bind:value={query}/>
        {/if}
        {#await search_tags()}
          <p>Loading...</p>
        {:then}
          <div class="left-row">
            <p class="thin-label">Add Tags...</p>
          </div>
         {#if !tags_list.map(tag => tag.title).includes(query)
          && !tags.map(tag => tag.title).includes(query)
          && query.trim().length > 2}
            <div class="left-row">
              <p style="color: #757474;">Create new tag: </p>
              <button on:click={add_new_tag_to_jot} class="tag">{query}</button>
            </div>
            
         {/if}
          
          {#each tags_list as tag, i}
            <button on:click={() => {
                add_tag_to_jot(tag.id)
                query = "";
                // force reload of tags list
                tags.push(tag);
                tags = [...tags];
                search_tags();
            }} class="tag">{tag.title}</button>
          {:else}
            <p>No tags found...</p>
          {/each}
        {:catch error}
          <p>Error: {error.message}</p>
        {/await}
      </div>
      
    </div>
  {/if}
  
  <style>
    .popup {
      position: fixed;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      background-color: rgba(0, 0, 0, 0.5);
      z-index: 9999;
      display: flex;
      justify-content: center;
      align-items: center;
    }
  
    .popup-content {
      background-color: #2f2f2f;
      border-radius: 10px;
      border: 2px solid #464444;
      padding: 5px;
      margin-top: 0em;
      width: 30em; 
      max-height: 40em;
      word-wrap: break-word;
      white-space: pre-line;
      text-align: left;
      box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
      position: relative;
      overflow-y: scroll;
    }
    
    .close-button {
      position: absolute;
      top: 0em;
      right: 0em;
      display: inline-block;
      padding: 0px 6px;
      border-radius: 50px;
      background-color: #383838;
      color: #757474;
      font-size: 20px;
      font-weight: bold;
      text-align: center;
      margin-right: 10px;
      margin-bottom: 10px;
      cursor: pointer;
    }
    .action-buttons {
      display: flex;

      justify-content: space-between;
    }
    .action-button {
      display: inline-block;
      padding: 0px 6px;
      border-radius: 50px;
      background-color: #383838;
      color: #757474;
      font-size: 20px;
      font-weight: bold;
      text-align: center;
      cursor: pointer;
      align-content: end;
    }
    button:disabled {
      background-color: #383838;
      color: #75747446;
    }
    textarea {
      border-radius: 10px;
      background-color: #7574741a;
      width: 28.7em;
      height: auto;
      font-size: 16px;
      color: white;
      border: 0em;
      padding: 10px;
      resize: none;
      overflow: hidden;
    }
  
  </style>
  