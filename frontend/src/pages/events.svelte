<script lang="ts">
  import { onMount } from "svelte";
  import EventCard from "../lib/components/event_card.svelte";
  import EventDetail from "../lib/components/event_detail.svelte";
  import { EventStatus, type Event, type Artist } from "../lib/types/data";

  let selectedEvent = $state<Event | null>(null);

  let events = $state<Event[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      const res = await fetch("/api/events");
      if (!res.ok) throw new Error("Failed to fetch events");
      events = await res.json();
    } catch (e) {
      error = e instanceof Error ? e.message : "Something went wrong";
    } finally {
      loading = false;
    }
  });

  function selectEvent(event: Event) {
    selectedEvent = event;
  }

  function deselectEvent() {
    selectedEvent = null;
  }
</script>

{#if selectedEvent}
  <EventDetail event={selectedEvent} onBack={deselectEvent} />
{:else}
  <div class="events-list">
    {#each events as event, i}
      <EventCard {event} index={i} onSelect={selectEvent} />
    {/each}
  </div>
{/if}
