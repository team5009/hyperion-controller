<script lang="ts">
    import { listen } from '@tauri-apps/api/event';
    import { onMount } from 'svelte';
    import {invoke} from '@tauri-apps/api/tauri';

  let ping = 0;
  onMount(async () => {
      await listen('bot_event', async (res) => {
          let time = Date.now();
          const { event, payload } = res as {
              event: string;
              payload: string;
            };
            
            const data = JSON.parse(payload);
            console.log(data);
            
            switch (data.event) {
                case 'pong':
                    ping = Date.now() - time;
                    time = Date.now()
                    const newMessage = JSON.stringify({
                        event: 'ping',
                    });
                    await invoke('send_socket_message', { message: newMessage});
                break;
            }
        })

    }); 

</script>

<h1>Ping: {ping}</h1>

<style>
  h1 {
    color: white;
    position: absolute;
    top: 1rem;
    right: 1rem;
  }
</style>