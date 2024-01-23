<script lang="ts">
    import Icon from '@iconify/svelte';
    import infoCircle from '@iconify/icons-majesticons/info-circle';
    import closeIcon from '@iconify/icons-majesticons/close';
    import { NotificationState } from '$store';
    import { ErrorType } from '$lib';
    import { onMount } from 'svelte';
  import { writable } from 'svelte/store';

    let notificationContainer: HTMLElement

    let Showing = writable("hide" as "hide" | "show")
    let showing = "hide"
    let notificationType = ""
    let notificationMessage = ""
    let notificationIcon = infoCircle


 
    NotificationState.subscribe((value) => {
        notificationMessage = value.message
        switch (value.type) {
            case ErrorType.SUCCESS:
                notificationType = "success"
                Showing.set("show")
                break;
            case ErrorType.ERROR:
                notificationType = "error"
                Showing.set("show")
                break;
            case ErrorType.INFO:
                notificationType = "info"
                Showing.set("show")
                break;
            case ErrorType.WARNING:
                notificationType = "warning"
                Showing.set("show")
                break;
            case ErrorType.CRITICAL:
                notificationType = "critical"
                Showing.set("show")
                break;
            case ErrorType.NOTHING:
                notificationType = ""
                Showing.set("hide")
                break;
        }
    })

    Showing.subscribe((value) => {
        showing = value
    })

    onMount(() => {
        let entered = false
        let timeout: number
        let clicked = false

        notificationContainer.addEventListener("mouseover", () => {
            if (timeout) {
                clearTimeout(timeout)
            }
            Showing.set("show")
            entered = true
            notificationContainer.addEventListener("mousedown", () => {
                clicked = true
            })
            
            notificationContainer.addEventListener("mouseup", () => {
                clicked = false
                if (!clicked) {
                    setTimeout(() => {
                        Showing.set("hide")
                        setTimeout(() => {
                            NotificationState.set({type: ErrorType.NOTHING, message: ""})
                        }, 300)
                    }, 1000)


                }
            })
        })

        notificationContainer.addEventListener("mouseleave", () => {
            entered = false
            if (!entered && !clicked) {
                timeout = setTimeout(() => {
                    Showing.set("hide")
                    setTimeout(() => {
                        NotificationState.set({type: ErrorType.NOTHING, message: ""})
                    }, 300)
                }, 3000)
            }
        })
    })

</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<section class={`${showing} ${notificationType}`} bind:this={notificationContainer}>
    <div class={"textbox"}>
        <h1>
            {notificationType}
        </h1>
        <p>
            {notificationMessage}
        </p>
    </div>
    <div class="icon">
        <Icon icon={infoCircle}/>
    </div>
</section>

<style lang="scss">
    section {
        z-index: 10000000;
        position: absolute;
        display: flex;
        bottom: 1rem;
        left: 1rem;
        box-sizing: border-box;
        padding: 1rem;
        border-radius: 0.5rem;
        cursor: pointer;
        transition: all 0.3s ease-in-out;
        max-width: calc(100% - 11rem);
        
        .textbox {
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: flex-start;
            h1 {
                font-size: 1.5rem;
                font-weight: bold;
                margin: 0;
                text-transform: capitalize;
            }
            p {
                font-size: 1rem;
                font-weight: normal;
                margin: 0;
            }
        
        }

        .icon {
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100%;
            width: 3rem;
            margin: 0 0.5rem;
            font-size: 3.5rem;
        }
    }

    section:hover  {
        transform: scale(1.05);
    }
    .show {
        opacity: 1;
        transform: translateX(0);
    }
    .hide {
        opacity: 0;
        transform: translateX(calc(-100% - 1rem));
    }
    .success {
        background-color: #00f37a;
        color: white;
    }
    .error {
        background-color: #ff2d2d;
        color: white;
    }
    .warning {
        background-color: #ff9f00;
        color: white;
    }
    .info {
        background-color: #00b4ff;
        color: white;
    }
    .critical {
        background-color: #ff2d2d;
        color: rgb(182, 0, 0);
    }
</style>