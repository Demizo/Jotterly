<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { focusTrap } from 'svelte-focus-trap';
    import { onMount } from 'svelte';
    import Dropdown from "$lib/Dropdown.svelte";
    
    export let visible = false;
    
    function closePopup() {
      visible = false;
      document.body.style.overflow = 'auto';
    }
    function handleEscDown(event:  KeyboardEvent) {
      if (visible && event.key === 'Escape') {
        closePopup();
      }
    }
    let output = "";
    let themes: string[] = [];
    
    let selectedTheme = "";
    async function getThemes() {
      themes = [];
      selectedTheme = JSON.parse(await invoke("get_settings")).theme;
      themes.push("Dark");
      themes.push("Light");
      let otherThemes: string[] = await invoke("fetch_all_themes");
      otherThemes.sort();
      otherThemes = otherThemes.filter((t) => t !== "Dark" && t !== "Light");
      for (const theme of otherThemes) {
        themes.push(theme);
      }
        
    }
    
    const root = document.documentElement;
    
    async function changeTheme(theme: string) {
        selectedTheme = theme;
        applyTheme();
        await invoke("set_setting", {key: "theme", val: selectedTheme});
    }
    async function applyTheme() {
      let theme_data: JSON = JSON.parse(await invoke("get_theme", {theme: selectedTheme}));
      if(theme_data.primary_color != undefined) root.style.setProperty('--primary-color', theme_data.primary_color);
      if(theme_data.negative_color != undefined) root.style.setProperty('--negative-color', theme_data.negative_color);
      if(theme_data.background_color != undefined) root.style.setProperty('--background-color', theme_data.background_color);
      if(theme_data.background_color_dark != undefined) root.style.setProperty('--background-color-dark', theme_data.background_color_dark);
      if(theme_data.background_color != undefined) root.style.setProperty('--background-color-trans', theme_data.background_color + "00");
      if(theme_data.background_highlight != undefined) root.style.setProperty('--background-highlight', theme_data.background_highlight);
      if(theme_data.font_color != undefined) root.style.setProperty('--font-color', theme_data.font_color);
      if(theme_data.font_color_secondary != undefined) root.style.setProperty('--font-color-secondary', theme_data.font_color_secondary);
      if(theme_data.highlight_thickness != undefined) root.style.setProperty('--highlight-thickness', theme_data.highlight_thickness);
    }

    let fontSize = 16;
    async function getFontSize() {
      fontSize = JSON.parse(await invoke("get_settings")).font_size;
    }
    async function changeFontSize() {
      root.style.setProperty('--font-size', fontSize.toString()+"px");
      await invoke("set_setting", {key: "font_size", val: fontSize.toString()});
    }
    async function increaseFontSize() {
      if(fontSize<20) fontSize++;
      await changeFontSize();
    }
    async function decreaseFontSize() {
      if(fontSize>12) fontSize--;
      await changeFontSize();
    }
    
    document.addEventListener('keydown', handleEscDown);
  </script>
  {#if visible}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="popup" tabindex="-1" on:mousedown={closePopup}>
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div class="popup-content" use:focusTrap on:mousedown|stopPropagation>
        <div class="action-buttons">
          <div></div>
          <button class="action-button" on:click={closePopup}>
            <svg class="exit-icon" xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000">
              <path d="M0 0h24v24H0V0z" fill="none"/><path d="M18.3 5.71c-.39-.39-1.02-.39-1.41 0L12 10.59 7.11 5.7c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41L10.59 12 5.7 16.89c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0L12 13.41l4.89 4.89c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L13.41 12l4.89-4.89c.38-.38.38-1.02 0-1.4z"/>
            </svg>
          </button>
        </div>
        <div class="row">
          <svg class="settings-icon" xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000">
            <g><rect fill="none" height="24" width="24"/></g><g><path d="M12,2C6.49,2,2,6.49,2,12s4.49,10,10,10c1.38,0,2.5-1.12,2.5-2.5c0-0.61-0.23-1.2-0.64-1.67c-0.08-0.1-0.13-0.21-0.13-0.33 c0-0.28,0.22-0.5,0.5-0.5H16c3.31,0,6-2.69,6-6C22,6.04,17.51,2,12,2z M17.5,13c-0.83,0-1.5-0.67-1.5-1.5c0-0.83,0.67-1.5,1.5-1.5 s1.5,0.67,1.5,1.5C19,12.33,18.33,13,17.5,13z M14.5,9C13.67,9,13,8.33,13,7.5C13,6.67,13.67,6,14.5,6S16,6.67,16,7.5 C16,8.33,15.33,9,14.5,9z M5,11.5C5,10.67,5.67,10,6.5,10S8,10.67,8,11.5C8,12.33,7.33,13,6.5,13S5,12.33,5,11.5z M11,7.5 C11,8.33,10.33,9,9.5,9S8,8.33,8,7.5C8,6.67,8.67,6,9.5,6S11,6.67,11,7.5z"/></g>
          </svg>
          <p class="settings-category-label">Appearance</p>
        </div>
        <div class="settings-item">
          {#await getThemes()}
            
          {:then}
          <p class="thin-label">Theme: </p>
          <div class="row">
            <button class="action-button" on:click={async () => {
              await invoke("open_themes_folder");
            }}>
              <svg class="save-icon" xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000">
                <path d="M0 0h24v24H0z" fill="none"/><path d="M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 12H4V8h16v10z"/>
              </svg>
            </button>
            <Dropdown bind:options={themes} bind:selectedOption={selectedTheme} onOptionSelect={changeTheme}></Dropdown>
          </div>
          {/await}
        </div>
        {#await getFontSize()}
          
        {:then} 
        <div class="settings-item">
          <p class="thin-label">Font Size:</p>
          <div class="row">
            
            <button on:click={decreaseFontSize}>-</button>
            <button tabindex="-1">{fontSize}</button>
            <button on:click={increaseFontSize}>+</button>
          </div>
        </div>
        {/await}

        <div class="row">
          <svg class="settings-icon" xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000"><path d="M0 0h24v24H0z" fill="none"/>
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"/>
          </svg>
          <p class="settings-category-label">About</p>
        </div>
        <div class="settings-item">
          <p class="thin-label">Version:</p>
          <button on:click={async () => {
            await invoke("open_link", {url: "https://github.com/Demizo/Jotterly/releases"});
          }}>
            <p>v1.0.0</p>
          </button>
        </div>
        <div class="settings-item">
          <p class="thin-label">Source Code:</p>
          <button on:click={async () => {
            await invoke("open_link", {url: "https://github.com/Demizo/Jotterly"});
          }}>
            <svg style="fill: var(--font-color);" width="24" height="24" viewBox=" 0 0 98 96" xmlns="http://www.w3.org/2000/svg">
              <path fill-rule="evenodd" clip-rule="evenodd" d="M48.854 0C21.839 0 0 22 0 49.217c0 21.756 13.993 40.172 33.405 46.69 2.427.49 3.316-1.059 3.316-2.362 0-1.141-.08-5.052-.08-9.127-13.59 2.934-16.42-5.867-16.42-5.867-2.184-5.704-5.42-7.17-5.42-7.17-4.448-3.015.324-3.015.324-3.015 4.934.326 7.523 5.052 7.523 5.052 4.367 7.496 11.404 5.378 14.235 4.074.404-3.178 1.699-5.378 3.074-6.6-10.839-1.141-22.243-5.378-22.243-24.283 0-5.378 1.94-9.778 5.014-13.2-.485-1.222-2.184-6.275.486-13.038 0 0 4.125-1.304 13.426 5.052a46.97 46.97 0 0 1 12.214-1.63c4.125 0 8.33.571 12.213 1.63 9.302-6.356 13.427-5.052 13.427-5.052 2.67 6.763.97 11.816.485 13.038 3.155 3.422 5.015 7.822 5.015 13.2 0 18.905-11.404 23.06-22.324 24.283 1.78 1.548 3.316 4.481 3.316 9.126 0 6.6-.08 11.897-.08 13.526 0 1.304.89 2.853 3.316 2.364 19.412-6.52 33.405-24.935 33.405-46.691C97.707 22 75.788 0 48.854 0z"/>
            </svg>
          </button>
        </div>
        <div class="settings-item">
          <p class="thin-label">License:</p>
          <button on:click={async () => {
            await invoke("open_link", {url: "https://github.com/Demizo/Jotterly/blob/master/LICENSE.txt"});
          }}>
            <p>GPL v3.0</p>
          </button>
        </div>
        
      </div>
    </div>
  {/if}
  
  <style>
  .settings-icon {
      fill: var(--primary-color);
  }
  .settings-category-label {
    font-weight: bold;
    color: var(--primary-color);
    margin: 0em;
  }
  .settings-item {
    padding-right: 1em;
    padding-left: 1em;
    padding-bottom: 1em;
    display: flex;
    justify-content: space-between;
  }
  </style>