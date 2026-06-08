<script lang="ts">
  import { EventStatus, type Event } from "../types/data";

  type Props = {
    event: Event;
    index?: number;
    onSelect?: (event: Event) => void;
  };

  let { event, index = 0, onSelect }: Props = $props();

  function handleClick() {
    onSelect?.(event);
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_to_interactive_role -->
<article
  class="event"
  onclick={handleClick}
  role="button"
  tabindex="0"
  onkeydown={(e) => e.key === "Enter" && handleClick()}
>
  <div class="event-index">
    <span class="idx-prefix">EV—</span><span class="idx-num"
      >{String(index + 1).padStart(3, "0")}</span
    >
  </div>
  <div class="event-date">
    <span class="date-main">{event.date}</span>
  </div>
  <div class="event-info">
    <h3 class="event-name">{event.name}</h3>
    <div class="event-meta">
      <span class="meta-label">VENUE</span>
      <span class="meta-value">{event.venue}</span>
    </div>
    <div class="event-meta">
      <span class="meta-label">LINEUP</span>
      <span class="meta-value">
        {#each event.lineup as artist}
          {artist.name},
        {/each}
      </span>
    </div>
  </div>
  <div class="event-status">
    {#if event.status === EventStatus.SoldOut}
      <span class="badge sold-out">
        <span class="badge-dot"></span>
        Sold out
      </span>
    {:else if event.status === EventStatus.ComingSoon}
      <span class="badge coming-soon">
        <span class="badge-dot"></span>
        Coming soon
      </span>
    {:else if event.status === EventStatus.Available}
      <span class="badge available">
        <span class="badge-dot"></span>
        Tickets available
      </span>
    {/if}
  </div>
</article>

<style>
  .event {
    display: grid;
    grid-template-columns: 64px 92px 1fr auto;
    gap: 0 24px;
    padding: 22px 4px;
    border-bottom: 1px solid var(--color-border);
    transition:
      background 0.25s,
      border-color 0.25s;
    align-items: start;
    position: relative;
  }

  /* faint left tick — like a margin mark in a notebook */
  .event::before {
    content: "";
    position: absolute;
    left: 0;
    top: 50%;
    width: 0;
    height: 1px;
    background: var(--color-highlight);
    transition: width 0.3s ease;
  }

  .event:hover {
    background: color-mix(in srgb, var(--color-secondary) 2%, transparent);
    border-bottom-color: color-mix(
      in srgb,
      var(--color-border) 50%,
      var(--color-secondary)
    );
  }

  .event:hover::before {
    width: 12px;
  }

  .event:hover .event-name {
    color: var(--color-secondary);
  }

  .event:hover .idx-num {
    color: var(--color-highlight);
  }

  /* ── Index ── */
  .event-index {
    font-family: var(--font-mono);
    font-size: 9px;
    letter-spacing: 0.18em;
    padding-top: 4px;
    line-height: 1.4;
    transition: color 0.25s;
  }
  .idx-prefix {
    color: var(--color-dim);
  }
  .idx-num {
    color: var(--color-muted);
    transition: color 0.25s;
  }

  /* ── Date ── */
  .event-date {
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 0.14em;
    color: var(--color-secondary);
    padding-top: 2px;
    line-height: 1.4;
    text-transform: uppercase;
  }

  /* ── Info block ── */
  .event-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .event-name {
    font-family: var(--font-display);
    font-size: 22px;
    font-weight: 400;
    line-height: 1.05;
    color: color-mix(in srgb, var(--color-secondary) 80%, transparent);
    margin: 0 0 8px;
    letter-spacing: 0.01em;
    text-transform: lowercase;
    transition: color 0.25s;
  }

  .event-meta {
    display: grid;
    grid-template-columns: 56px 1fr;
    gap: 12px;
    font-family: var(--font-mono);
    font-size: 9px;
    letter-spacing: 0.18em;
    text-transform: uppercase;
    line-height: 1.6;
  }

  .meta-label {
    color: var(--color-dim);
  }

  .meta-value {
    color: var(--color-muted);
    letter-spacing: 0.08em;
  }

  /* ── Badge ── */
  .badge {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    font-family: var(--font-mono);
    font-size: 9px;
    letter-spacing: 0.22em;
    padding: 5px 10px;
    border: 1px solid;
    text-transform: uppercase;
    white-space: nowrap;
    margin-top: 4px;
  }

  .badge-dot {
    width: 5px;
    height: 5px;
    border-radius: 50%;
    background: currentColor;
  }

  .available {
    border-color: color-mix(in srgb, var(--color-accent) 40%, transparent);
    color: var(--color-accent);
  }
  .available .badge-dot {
    animation: pulse 2s ease-in-out infinite;
  }

  .sold-out {
    border-color: color-mix(in srgb, var(--color-highlight) 35%, transparent);
    color: color-mix(in srgb, var(--color-highlight) 75%, var(--color-dim));
  }
  .sold-out .badge-dot {
    background: var(--color-highlight);
    opacity: 0.5;
  }

  .coming-soon {
    border-color: var(--color-border);
    color: var(--color-muted);
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.4;
    }
  }

  /* ── Mobile ── */
  @media (max-width: 640px) {
    .event {
      grid-template-columns: 1fr auto;
      grid-template-areas:
        "index    status"
        "date     date"
        "info     info";
      gap: 8px 12px;
      padding: 20px 4px;
    }
    .event-index {
      grid-area: index;
    }
    .event-date {
      grid-area: date;
      padding-top: 0;
    }
    .event-info {
      grid-area: info;
    }
    .event-status {
      grid-area: status;
    }
  }
</style>
