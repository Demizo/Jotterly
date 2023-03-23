<script lang="ts">
  export let options: string[] = [];
  export let selectedOption = "";
  export let onOptionSelect: (option: string) => {};
  
  let dropdownOpen = false;

  function toggleDropdown() {
    dropdownOpen = !dropdownOpen;
  }
  
  function selectOption(option: string) {
    selectedOption = option;
    onOptionSelect(selectedOption);
    dropdownOpen = false;
  }
</script>

<div class="dropdown">
  <button class="dropdown-header" on:click={toggleDropdown}>
    <span>{selectedOption || "Select an option"}</span>
    <svg viewBox="0 0 24 24" class={`icon ${dropdownOpen ? "rotate-180" : ""}`}>
      <path d="M7 10l5 5 5-5z" />
    </svg>
  </button>
  {#if dropdownOpen}
    <div class="dropdown-options">
      {#each options as option}
        <button class="dropdown-option" on:click={() => selectOption(option)}>
          {option}
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .dropdown {
    position: relative;
  }

  .dropdown-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background-color: var(--background-color-dark);
    border-radius: 10px;
    padding: 0.5rem;
  }
  
  .dropdown-header:hover {
    background-color: var(--background-color-dark);
    border: var(--highlight-thickness) solid var(--primary-color);
  }
  
  .icon {
    width: 1rem;
    height: 1rem;
    fill: var(--font-color);
    transition: transform 0.3s;
  }

  .rotate-180 {
    transform: rotate(180deg);
  }

  .dropdown-options {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background-color: var(--background-color-dark);
    border: var(--highlight-thickness) solid var(--background-color-dark);
    border-top: none;
    border-radius: 10px;
    overflow: hidden;
    z-index: 1;
  }

  .dropdown-option {
    padding: 0.5rem;
    cursor: pointer;
    box-shadow: none;
    width: 100%;
  }
</style>
