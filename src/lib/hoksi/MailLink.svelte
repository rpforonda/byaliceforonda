<script>
  import { EMAIL_USER, EMAIL_DOMAIN } from "#lib/hoksi/data.js";

  let { class: className = "" } = $props();

  // The prerendered HTML only says "hoksihaus at gmail.com". The mailto: is put on the link when
  // someone points at, focuses or taps it, so bots reading the page find no name@domain.
  const mailto = () => `mailto:${EMAIL_USER}@${EMAIL_DOMAIN}`;

  // set on the element directly, so a click straight after focus already follows the mailto:
  function reveal(event) {
    event.currentTarget.setAttribute("href", mailto());
  }

  function handleClick(event) {
    // nothing revealed the link first (unusual): open mail directly
    if (event.currentTarget.getAttribute("href") === "#") {
      event.preventDefault();
      window.location.href = mailto();
    }
  }
</script>

<!-- "#" is only a placeholder: reveal() swaps in the mailto: before any click goes through -->
<!-- svelte-ignore a11y_invalid_attribute -->
<a
  href="#"
  class={className}
  onpointerenter={reveal}
  onpointerdown={reveal}
  onfocus={reveal}
  onclick={handleClick}
>{EMAIL_USER} at {EMAIL_DOMAIN}</a>
