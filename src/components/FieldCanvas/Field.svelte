<script>
    import {resolution} from '$lib'
    import FieldMap from './FieldMap.svelte'
    import FieldCanvas from './FieldCanvas.svelte'

</script>
<div>
    <FieldCanvas resolution={resolution}/>
    <FieldMap/>
</div>

<style>
    div {
        position: relative;
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>