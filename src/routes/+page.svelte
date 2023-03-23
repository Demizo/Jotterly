<script land="ts">
  import JotBar from "$lib/JotBar.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  
  const root = document.documentElement;
  async function initUI() {
      let theme = JSON.parse(await invoke("get_settings")).theme;
      console.log(theme);
      let theme_data = JSON.parse(await invoke("get_theme", {theme: theme}));
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

</script>

{#await initUI()}
  
{:then} 
  <div class="row"></div>
  <JotBar/>
{/await}
