<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { focusTrap } from 'svelte-focus-trap';

    export let focusText = true;
    export let visible = false;
    export let jotId: Number;
    export let jot = {id: Number, text: String, img_path: String, time_create: String, time_modified: String };
    export let search_jots;
    export let tags = [{id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}];
    let temp_tags = tags.slice(0, tags.length);
    let query = "";
    let pending = false;
    let text = jot.text;
    let temptext = text;
    
    function closePopup() {
      query = "";
      temptext = text;
      temp_tags = tags.slice(0, tags.length);
      visible = false;
    }
    let tags_list: {id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}[];
    async function search_tags() {
      if (query.trim().length > 2 || query.trim().length == 0 && pending == false) {
        let tag_ids = temp_tags.map(tag => tag.id);
        tags_list = await invoke("search_tags", {query: query, tagIds: tag_ids}).catch(() => console.log("failed to get tags")) as {id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}[];
        tags_list = tags_list.filter(t => !temp_tags.map(tag => tag.title).includes(t.title));
      }
    }
    async function add_new_tag_to_jot() {
      temp_tags.push({id: -1, title: query, color: "", priority: 0, time_create: "", time_modified: ""});
      temp_tags = [...temp_tags];
      query = "";
      search_tags();
    }
    function add_tag(tag){
        temp_tags.push(tag);
        temp_tags = [...temp_tags];
    }
    function remove_tag(title: String){
      temp_tags = temp_tags.filter(t => t.title != title);
    }
    async function save_jot_tags() {
        let new_tags = temp_tags.filter((t) => t.id == -1);
        //filter out new tags
        temp_tags = temp_tags.filter((t) => t.id != -1);
        
        let remove_tags = tags.filter((t) => !temp_tags.map((tag) => tag.title).includes(t.title));
        let add_tags = temp_tags.filter((t) => !tags.map((tag) => tag.title).includes(t.title));

        //remove tags
        for(let i = 0; i < remove_tags.length; i++) {
          await invoke("remove_tag_from_jot", {tagId: remove_tags[i].id, jotId: jotId}).catch(() => console.log("failed to remove tag"));
        }
        
        //add tags
        for(let i = 0; i < add_tags.length; i++) {
          await invoke("add_tag_to_jot", {tagId: add_tags[i].id, jotId: jotId}).catch(() => console.log("failed to add tag"));
        }
        
        //create and add new tags
        for(let i = 0; i < new_tags.length; i++) {
          let tag = await invoke("add_new_tag_to_jot", {title: new_tags[i].title, jotId: jotId}).catch(() => console.log("failed to add new tag")) as {id: NumberConstructor, title: StringConstructor, color: StringConstructor, priority: NumberConstructor, time_create: StringConstructor, time_modified: StringConstructor};
          add_tag(tag);
        }
        
    }
    async function saveChanges() {
      text = temptext;
      await save_jot_tags();
      await save_jot_text();
      jot.text = text;
      tags = temp_tags.slice(0, temp_tags.length);
      await search_tags();
    }
    async function save_jot_text() {
      await invoke("update_jot_text", {id: jot.id, text: text, img_path: jot.img_path});
    }
    async function delete_jot() {
      await invoke("delete_jot", {id: jot.id});
      search_jots();
    }
    
  </script>
  
  {#if visible}
    <div class="popup">
      <div class="popup-content" use:focusTrap>
        <div class="action-buttons">
          <button class="action-button" on:click={delete_jot}>d</button>
          <div>
            <button disabled={temptext.toString().trim().length <= 0 || (temptext === text && (temp_tags.length === tags.length && tags.every((t) => temp_tags.includes(t))))} class="action-button" on:click={saveChanges}>s</button>
            <button class="action-button" on:click={closePopup}>x</button>
          </div>
        </div>
        {#if focusText}
          <textarea autofocus bind:value={temptext}></textarea>
        {:else}
        <textarea bind:value={temptext}></textarea>
        {/if}
        <div class="left-row">
          <p class="thin-label">Remove Tags...</p>
        </div>
        {#each temp_tags as tag, i}
          <button on:click={() => {
            remove_tag(tag.title);
            search_tags();
          }} class="tag">{tag.title}</button>
        {:else}
          <p>No tags...</p>
        {/each}
        {#if focusText}
          <input style="width: 27em; margin-bottom: 1em;" inputmode="search" on:keyup={search_tags} placeholder="Search Tags..." bind:value={query}/>
        {:else}
          <input autofocus style="width: 27em; margin-bottom: 1em;" inputmode="search" on:keyup={search_tags} placeholder="Search Tags..." bind:value={query}/>
        {/if}
        {#await search_tags()}
          <p>Loading...</p>
        {:then}
          <div class="left-row">
            <p class="thin-label">Add Tags...</p>
          </div>
         {#if !tags_list.map(tag => tag.title).includes(query)
          && !temp_tags.map(tag => tag.title).includes(query)
          && query.trim().length > 2}
            <div class="left-row">
              <p style="color: var(--foreground-font-color-secondary);">Create new tag: </p>
              <button on:click={add_new_tag_to_jot} class="tag">{query}</button>
            </div>
            
         {/if}
          
          {#each tags_list as tag, i}
            <button on:click={() => {
                add_tag(tag);
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
  