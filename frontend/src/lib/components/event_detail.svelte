<script lang="ts">
  import { EventStatus, type Event } from "../types/data";
  import { formatDate } from "../utils/formats";

  type Props = {
    event: Event;
    onBack?: () => void;
  };

  let { event, onBack }: Props = $props();

  function statusLabel(status: EventStatus): string {
    if (status === EventStatus.SoldOut) return "● SOLD OUT";
    if (status === EventStatus.ComingSoon) return "○ COMING SOON";
    if (status === EventStatus.Available) return "● TICKETS AVAILABLE";
    return "";
  }

  function statusColor(status: EventStatus): string {
    if (status === EventStatus.SoldOut) return "var(--color-highlight)";
    if (status === EventStatus.Available) return "var(--color-accent)";
    return "var(--color-muted)";
  }
</script>

<article class="detail">
  <header class="detail-header">
    <button class="back-btn" onclick={onBack}>
      <span class="back-arrow">←</span>
      <span>BACK TO INDEX</span>
    </button>
    <div class="header-spec">
      SPEC. № EV—001 <span class="dot">/</span> PG.01 OF 04
    </div>
  </header>

  <div class="detail-body">
    <aside class="meta-column">
      <div class="meta-row">
        <span class="meta-label">DATE</span>
        <span class="meta-value">{formatDate(event.date)}</span>
      </div>
      <div class="meta-row">
        <span class="meta-label">VENUE</span>
        <span class="meta-value">{event.venue}</span>
      </div>

      <div class="meta-status" style="color: {statusColor(event.status)};">
        {statusLabel(event.status)}
      </div>
    </aside>

    <main class="content-column">
      <h1 class="event-title">{event.name}</h1>
      <p class="event-presents">— BLINDSPOT PRESENTS —</p>

      <section class="photo-stack">
        <figure class="photo photo-a">
          <div class="photo-placeholder">
            <span>PHOTO 01 →</span>
          </div>
          <figcaption>FIELD CAPTURE / FRAME 01</figcaption>
        </figure>
        <figure class="photo photo-b">
          <div class="photo-placeholder">
            <span>PHOTO 02 →</span>
          </div>
          <figcaption>FIELD CAPTURE / FRAME 02</figcaption>
        </figure>
        <figure class="photo photo-c">
          <div class="photo-placeholder">
            <span>PHOTO 03 →</span>
          </div>
          <figcaption>FIELD CAPTURE / FRAME 03</figcaption>
        </figure>
      </section>

      <section class="lineup-block">
        <div class="lineup-label">LINEUP</div>
        <div class="lineup-list">
          {#each event.lineup as artist}
            <div class="lineup-artist">{artist.name}</div>
          {/each}
        </div>
      </section>

      <section class="notes-block">
        <div class="notes-label">OBS. NOTES</div>
        <p class="notes-body">
          [ Field notes for this event will appear here. Annotations,
          observations, and recovered fragments from the night. ]
        </p>
      </section>
    </main>
  </div>

  <footer class="detail-footer">
    <span>OBS. {formatDate(event.date)}</span>
    <span>— HIVE—MIND —</span>
    <span>SPEC. № EV—001</span>
  </footer>
</article>

<style>
  .detail {
    max-width: 1100px;
    margin: 0 auto;
    padding: 8px 0 80px;
    color: var(--color-secondary);
    font-family: var(--font-mono);
  }

  /* ── Header ── */
  .detail-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 16px;
    margin-bottom: 32px;
    border-bottom: 1px solid var(--color-border);
    font-size: 9px;
    letter-spacing: 0.3em;
    text-transform: uppercase;
    color: var(--color-muted);
  }

  .back-btn {
    display: flex;
    align-items: center;
    gap: 12px;
    background: none;
    border: none;
    cursor: none;
    color: var(--color-secondary);
    font-family: var(--font-mono);
    font-size: 9px;
    letter-spacing: 0.3em;
    text-transform: uppercase;
    padding: 4px 8px;
    transition: color 0.2s;
  }

  .back-btn:hover {
    color: var(--color-highlight);
  }

  .back-arrow {
    transition: transform 0.2s;
  }

  .back-btn:hover .back-arrow {
    transform: translateX(-4px);
  }

  .header-spec .dot {
    color: var(--color-dim);
    margin: 0 6px;
  }

  /* ── Body grid: meta column + content column ── */
  .detail-body {
    display: grid;
    grid-template-columns: 140px 1fr;
    gap: 48px;
    align-items: start;
  }

  /* ── Meta column (sticks to the left margin) ── */
  .meta-column {
    position: sticky;
    top: 28px;
    display: flex;
    flex-direction: column;
    gap: 18px;
    font-size: 9px;
    letter-spacing: 0.18em;
    text-transform: uppercase;
  }

  .meta-row {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .meta-label {
    color: var(--color-dim);
  }

  .meta-value {
    color: var(--color-secondary);
  }

  .meta-status {
    margin-top: 8px;
    font-size: 9px;
    letter-spacing: 0.22em;
    padding-top: 12px;
    border-top: 1px solid var(--color-border);
  }

  /* ── Content column ── */
  .content-column {
    min-width: 0;
  }

  .event-title {
    font-family: var(--font-display);
    font-size: clamp(40px, 6vw, 64px);
    line-height: 0.92;
    font-weight: 400;
    color: var(--color-secondary);
    text-transform: lowercase;
    letter-spacing: 0.01em;
    margin: 0 0 8px;
  }

  .event-presents {
    font-size: 9px;
    letter-spacing: 0.4em;
    text-transform: uppercase;
    color: var(--color-muted);
    margin: 0 0 56px;
  }

  /* ── Photo stack (broken grid) ── */
  .photo-stack {
    display: grid;
    grid-template-columns: repeat(12, 1fr);
    gap: 16px;
    margin-bottom: 64px;
  }

  .photo {
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .photo-a {
    grid-column: 1 / span 7;
  }
  .photo-b {
    grid-column: 8 / span 5;
    margin-top: 56px;
  }
  .photo-c {
    grid-column: 3 / span 6;
    margin-top: 24px;
  }

  .photo-placeholder {
    width: 100%;
    aspect-ratio: 4 / 5;
    background: color-mix(in srgb, var(--color-secondary) 4%, transparent);
    border: 1px solid var(--color-border);
    display: flex;
    align-items: end;
    justify-content: start;
    padding: 12px;
    font-size: 9px;
    letter-spacing: 0.2em;
    color: var(--color-dim);
  }

  .photo figcaption {
    font-size: 8px;
    letter-spacing: 0.28em;
    color: var(--color-dim);
    text-transform: uppercase;
  }

  /* ── Lineup ── */
  .lineup-block {
    display: grid;
    grid-template-columns: 80px 1fr;
    gap: 24px;
    margin-bottom: 56px;
    padding-top: 32px;
    border-top: 1px solid var(--color-border);
  }

  .lineup-label {
    font-size: 9px;
    letter-spacing: 0.3em;
    color: var(--color-dim);
    text-transform: uppercase;
    padding-top: 4px;
  }

  .lineup-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .lineup-artist {
    display: grid;
    align-items: baseline;
    gap: 12px;
  }

  .lineup-idx {
    font-size: 9px;
    letter-spacing: 0.18em;
    color: var(--color-dim);
  }

  .lineup-name {
    font-family: var(--font-display);
    font-size: 18px;
    color: var(--color-secondary);
    text-transform: lowercase;
    letter-spacing: 0.02em;
  }

  /* ── Notes ── */
  .notes-block {
    display: grid;
    grid-template-columns: 80px 1fr;
    gap: 24px;
    padding-top: 32px;
    border-top: 1px solid var(--color-border);
  }

  .notes-label {
    font-size: 9px;
    letter-spacing: 0.3em;
    color: var(--color-dim);
    text-transform: uppercase;
    padding-top: 4px;
  }

  .notes-body {
    font-size: 12px;
    line-height: 1.8;
    color: var(--color-muted);
    margin: 0;
    max-width: 520px;
  }

  /* ── Footer ── */
  .detail-footer {
    margin-top: 80px;
    padding-top: 16px;
    border-top: 1px solid var(--color-border);
    display: flex;
    justify-content: space-between;
    font-size: 8px;
    letter-spacing: 0.3em;
    color: var(--color-dim);
    text-transform: uppercase;
  }

  /* ── Mobile ── */
  @media (max-width: 720px) {
    .detail-body {
      grid-template-columns: 1fr;
      gap: 32px;
    }
    .meta-column {
      position: static;
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 16px;
    }
    .photo-stack {
      grid-template-columns: 1fr;
    }
    .photo-a,
    .photo-b,
    .photo-c {
      grid-column: 1;
      margin-top: 0;
    }
    .lineup-block,
    .notes-block {
      grid-template-columns: 1fr;
    }
  }
</style>
