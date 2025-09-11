<script lang="ts">
    import { data } from "$lib/store";
  import { invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";
  import { SvelteMap } from "svelte/reactivity";
    
    type PlayerStats = {
        id: number;
        name: string;
        damageDealt: number;
        damageReceived: number;
    };
    
    let players = $state(new SvelteMap<number, PlayerStats>());
    let isRunning = $state(false);
    let totalDamage = $state(0);
    let zoneCount = $state(0);
    let unsubscribe: () => void;
    let clearHandle: number;

    onMount(() => {

        clearHandle = setInterval(async () => {

            isRunning = await invoke("heartbeat");

        }, 10000);

        unsubscribe = data.subscribe(($data) => {
            if (!$data) return;
            
            const event = $data as any;
            isRunning = true;
            
            if (event["app-event"] === "ZoneChange") {

                players.clear();
                totalDamage = 0;
                zoneCount++;
            } 
            else if (event["app-event"] === "Spawn") {

                if (!players.has(event.data.id)) {
                    players.set(event.data.id, {
                        id: event.data.id,
                        name: event.data.name,
                        damageDealt: 0,
                        damageReceived: 0
                    });
                } else {
                    players.get(event.data.id)!.name = event.data.name;
                }
            }
            else if (event["app-event"] === "Damage") {

                totalDamage += event.data.value;
          
                if (players.has(event.data.id)) {
                    const stats = players.get(event.data.id)!;
                    stats.damageDealt += totalDamage;
                    players.set(event.data.id, {
                        ...stats
                    });
                }

            }
        });
    })
    
    onDestroy(() => unsubscribe?.());
    
    
</script>

<div class="stats-dashboard">
    <div class="status-banner" style="background-color: {isRunning ? '#a3f7a3' : '#f7a3a3'}">
        {isRunning ? "Background Worker Running" : "Background Worker Stopped"}
    </div>
    
    <div class="header">
        <span class="title">Battle Stats</span>
        <span class="zone-count">Zone changes: {zoneCount}</span>
    </div>
    
    <div class="total-damage">
        <span class="label">Total Damage:</span>
        <span class="value">{totalDamage}</span>
    </div>
    
    <div class="players-grid">
        {#each players.entries() as [id, player]}
            <div class="player-card">
                <div class="player-name">{player.name}</div>
                <div class="player-stats">
                    <span class="stat">
                        <span class="stat-label">Dealt:</span>
                        <span class="stat-value">{player.damageDealt}</span>
                    </span>
                </div>
            </div>
        {/each}
    </div>
    
    {#if players.size === 0}
        <div class="empty-state">
            <span class="empty-icon">🎮</span>
            <span class="empty-text">Waiting for players...</span>
        </div>
    {/if}
    
    <div class="footer">
        <span class="player-count">{players.size}/4 players</span>
    </div>
</div>

<style>
    .status-banner {
        transition: background-color 0.3s ease;
    }

    .stats-dashboard {
        width: 500px;
        height: 400px;
        background: #1a1a1a;
        color: white;
        border-radius: 8px;
        padding: 12px;
        display: flex;
        flex-direction: column;
        font-family: 'Arial', sans-serif;
    }
    
    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 10px;
        padding-bottom: 8px;
        border-bottom: 1px solid #333;
    }
    
    .title {
        font-size: 16px;
        font-weight: bold;
        color: #00ff88;
    }
    
    .zone-count {
        font-size: 12px;
        color: #888;
        background: #222;
        padding: 2px 8px;
        border-radius: 10px;
    }
    
    .total-damage {
        background: #2a2a2a;
        padding: 8px;
        border-radius: 6px;
        margin-bottom: 12px;
        text-align: center;
    }
    
    .label {
        color: #ccc;
        font-size: 12px;
        margin-right: 8px;
    }
    
    .value {
        color: #ff4444;
        font-size: 18px;
        font-weight: bold;
    }
    
    .players-grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: 1fr 1fr;
        gap: 8px;
        flex: 1;
        overflow-y: auto;
    }
    
    .player-card {
        background: #2a2a2a;
        padding: 8px;
        border-radius: 6px;
        border-left: 3px solid #00ff88;
    }
    
    .player-name {
        font-size: 12px;
        font-weight: bold;
        color: #00ff88;
        margin-bottom: 6px;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }
    
    .player-stats {
        display: flex;
        flex-direction: column;
        gap: 3px;
    }
    
    .stat {
        display: flex;
        justify-content: space-between;
        font-size: 11px;
    }
    
    .stat-label {
        color: #888;
    }
    
    .stat-value {
        color: #ff4444;
        font-weight: bold;
    }
    
    .empty-state {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        color: #666;
    }
    
    .empty-icon {
        font-size: 24px;
        margin-bottom: 8px;
    }
    
    .empty-text {
        font-size: 12px;
    }
    
    .footer {
        margin-top: 8px;
        padding-top: 8px;
        border-top: 1px solid #333;
        text-align: center;
    }
    
    .player-count {
        font-size: 11px;
        color: #888;
    }
    
    /* Scrollbar styling */
    .players-grid::-webkit-scrollbar {
        width: 3px;
    }
    
    .players-grid::-webkit-scrollbar-track {
        background: #1a1a1a;
    }
    
    .players-grid::-webkit-scrollbar-thumb {
        background: #00ff88;
        border-radius: 2px;
    }
</style>